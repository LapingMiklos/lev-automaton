use std::{
    collections::{BTreeMap, BTreeSet},
    marker::PhantomData,
    ops::{Index, IndexMut},
};

type Set<T> = BTreeSet<T>;
type Map<K, V> = BTreeMap<K, V>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateId(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transition {
    Is(char),
    IsNot(Set<char>),
    Star,
    Epsilon,
}

impl Transition {
    pub fn allows(&self, c: char) -> bool {
        match self {
            Self::Is(cc) => c == *cc,
            Self::IsNot(cs) => !cs.contains(&c),
            Self::Star => true,
            Self::Epsilon => false,
        }
    }

    pub fn is_epsilon(&self) -> bool {
        matches!(self, Self::Epsilon)
    }

    pub fn is_star(&self) -> bool {
        matches!(self, Self::Star)
    }
}

#[derive(Debug)]
pub struct State {
    transtions: Vec<(Transition, StateId)>,
}

impl State {
    const fn new() -> Self {
        Self { transtions: vec![] }
    }
}

#[derive(Debug)]
pub enum NonDeterministic {}

#[derive(Debug)]
pub enum Deterministic {}

#[derive(Debug)]
pub struct Automaton<T> {
    states: Vec<State>,
    start: Option<StateId>,
    final_states: Set<StateId>,
    _determinism_marker: PhantomData<T>,
}

impl<T> Default for Automaton<T> {
    fn default() -> Self {
        Self {
            states: vec![],
            start: None,
            final_states: Set::new(),
            _determinism_marker: PhantomData,
        }
    }
}

impl<T> Index<StateId> for Automaton<T> {
    type Output = State;

    fn index(&self, index: StateId) -> &Self::Output {
        &self.states[index.0]
    }
}

impl<T> IndexMut<StateId> for Automaton<T> {
    fn index_mut(&mut self, index: StateId) -> &mut Self::Output {
        &mut self.states[index.0]
    }
}

impl<T> Automaton<T> {
    pub fn add_state(&mut self) -> StateId {
        self.states.push(State::new());
        StateId(self.states.len() - 1)
    }

    pub const fn set_start_state(&mut self, index: StateId) {
        self.start = Some(index);
    }

    pub fn make_state_final(&mut self, index: StateId) {
        self.final_states.insert(index);
    }

    fn get_reachable_states<P>(&self, from: StateId, pred: P) -> impl Iterator<Item = StateId>
    where
        P: Fn(&Transition) -> bool,
    {
        self[from]
            .transtions
            .iter()
            .filter(move |(transition, _)| pred(transition))
            .map(|(_, to)| *to)
    }
}

impl Automaton<NonDeterministic> {
    pub fn add_transition(&mut self, from: StateId, to: StateId, transition: Transition) {
        self[from].transtions.push((transition, to));
    }

    pub fn run(&self, word: &str) -> bool {
        if self.states.is_empty() {
            return false;
        }

        let mut active_states = self.eps_closure(Set::from([self.start.unwrap_or(StateId(0))]));

        for c in word.chars() {
            let mut new_states = Set::new();
            for current_state in &active_states {
                for s in self.get_reachable_states(*current_state, |t| t.allows(c)) {
                    new_states.insert(s);
                }
            }

            if new_states.is_empty() {
                return false;
            }

            active_states = self.eps_closure(new_states);
        }

        active_states.iter().any(|s| self.final_states.contains(s))
    }

    fn eps_closure(&self, mut states: Set<StateId>) -> Set<StateId> {
        let mut new_states: Vec<StateId> = states.iter().copied().collect();

        while let Some(state) = new_states.pop() {
            for s in self.get_reachable_states(state, Transition::is_epsilon) {
                if states.insert(s) {
                    new_states.push(s);
                }
            }
        }

        states
    }
}

impl From<Automaton<NonDeterministic>> for Automaton<Deterministic> {
    fn from(nfa: Automaton<NonDeterministic>) -> Self {
        let mut dfa = Self::default();

        if nfa.states.is_empty() {
            return dfa;
        }

        let start_state = nfa.start.unwrap_or(StateId(0));
        let new_start_state = nfa.eps_closure(Set::from([start_state]));

        let mut state_map = Map::new();
        let mut state_stack = vec![new_start_state];

        while let Some(current_state) = state_stack.pop() {
            let dfa_from: StateId = *state_map
                .entry(current_state.clone())
                .or_insert_with(|| dfa.add_state());

            let chars: Set<char> = current_state
                .iter()
                .flat_map(|s| {
                    nfa[*s]
                        .transtions
                        .iter()
                        .filter_map(|(transition, _)| match transition {
                            Transition::Is(c) => Some(*c),
                            _ => None,
                        })
                })
                .collect();

            for c in &chars {
                let reachable_states: Set<StateId> = current_state
                    .iter()
                    .flat_map(|s| nfa.get_reachable_states(*s, |t| t.allows(*c)))
                    .collect();

                if reachable_states.is_empty() {
                    continue;
                }
                let reachabe_states = nfa.eps_closure(reachable_states);
                let dfa_to: StateId =
                    *state_map.entry(reachabe_states.clone()).or_insert_with(|| {
                        state_stack.push(reachabe_states);
                        dfa.add_state()
                    });

                unsafe {
                    dfa.add_transition_unchecked(dfa_from, dfa_to, Transition::Is(*c));
                }
            }

            let reachable_states: Set<StateId> = current_state
                .iter()
                .flat_map(|s| nfa.get_reachable_states(*s, Transition::is_star))
                .collect();

            if reachable_states.is_empty() {
                continue;
            }

            let reachabe_states = nfa.eps_closure(reachable_states);
            let dfa_to: StateId = *state_map.entry(reachabe_states.clone()).or_insert_with(|| {
                state_stack.push(reachabe_states);
                dfa.add_state()
            });

            unsafe {
                dfa.add_transition_unchecked(dfa_from, dfa_to, Transition::IsNot(chars));
            }
        }

        for (nfa_states, dfa_state) in state_map {
            if nfa_states.iter().any(|s| nfa.final_states.contains(s)) {
                dfa.make_state_final(dfa_state);
            }
        }

        dfa
    }
}

impl Automaton<Deterministic> {
    /// # Safety
    /// The caller is responsible for ensuring that adding the transition does not break determinism
    pub unsafe fn add_transition_unchecked(
        &mut self,
        from: StateId,
        to: StateId,
        transition: Transition,
    ) {
        self[from].transtions.push((transition, to));
    }

    pub fn run(&self, word: &str) -> bool {
        if self.states.is_empty() {
            return false;
        }

        let mut active_state = self.start.unwrap_or(StateId(0));

        for c in word.chars() {
            if let Some(new_state) = self[active_state]
                .transtions
                .iter()
                .filter(|(transition, _)| transition.allows(c))
                .map(|(_, to)| to)
                .next()
            {
                active_state = *new_state
            } else {
                return false;
            }
        }

        self.final_states.contains(&active_state)
    }
}
