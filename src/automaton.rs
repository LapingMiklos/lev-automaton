use std::{
    collections::{BTreeMap, BTreeSet},
    marker::PhantomData,
    ops::{Index, IndexMut},
};

type Set<T> = BTreeSet<T>;
type Map<K, V> = BTreeMap<K, V>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    fn _add_transition(&mut self, from: StateId, to: StateId, transition: Transition) {
        self[from].transtions.push((transition, to));
    }
}

impl Automaton<NonDeterministic> {
    pub fn add_transition(&mut self, from: StateId, to: StateId, transition: Transition) {
        self._add_transition(from, to, transition);
    }

    pub fn run(&self, word: &str) -> bool {
        if self.states.is_empty() {
            return false;
        }

        let mut active_states = Set::new();
        active_states.insert(self.start.unwrap_or(StateId(0)));

        for c in word.chars() {
            for state in &active_states.clone() {
                self.add_eps_states(&mut active_states, *state);
            }

            let mut new_states = Set::new();
            for current_state in &active_states {
                for s in self[*current_state]
                    .transtions
                    .iter()
                    .filter(|(transition, _)| transition.allows(c))
                    .map(|(_, to)| to)
                    .copied()
                {
                    new_states.insert(s);
                }
            }

            if new_states.is_empty() {
                return false;
            }

            active_states = new_states;
        }

        for state in &active_states.clone() {
            self.add_eps_states(&mut active_states, *state);
        }

        active_states.iter().any(|s| self.final_states.contains(s))
    }

    fn add_eps_states(&self, new_states: &mut Set<StateId>, current_state: StateId) {
        for s in self[current_state]
            .transtions
            .iter()
            .filter(|(transition, _)| transition.is_epsilon())
            .map(|(_, to)| to)
            .copied()
        {
            if !new_states.insert(s) {
                continue;
            }
            self.add_eps_states(new_states, s)
        }
    }

    fn eps_closure(&self, states: Set<StateId>) -> Set<StateId> {
        let mut new_states = states.clone();

        for s in states {
            self.add_eps_states(&mut new_states, s);
        }

        new_states
    }
}

impl From<Automaton<NonDeterministic>> for Automaton<Deterministic> {
    fn from(nfa: Automaton<NonDeterministic>) -> Self {
        let mut dfa = Self::default();

        if nfa.states.is_empty() {
            return dfa;
        }

        let start_state = nfa.start.unwrap_or(StateId(0));
        let mut new_start_state = Set::from([start_state]);
        nfa.add_eps_states(&mut new_start_state, start_state);

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
                    .flat_map(|s| {
                        nfa[*s]
                            .transtions
                            .iter()
                            .filter(|(transition, _)| transition.allows(*c))
                            .map(|(_, to)| to)
                    })
                    .copied()
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

                dfa._add_transition(dfa_from, dfa_to, Transition::Is(*c));
            }

            let reachable_states: Set<StateId> = current_state
                .iter()
                .flat_map(|s| {
                    nfa[*s]
                        .transtions
                        .iter()
                        .filter(|(transition, _)| transition.is_star())
                        .map(|(_, to)| to)
                })
                .copied()
                .collect();

            if reachable_states.is_empty() {
                continue;
            }

            let reachabe_states = nfa.eps_closure(reachable_states);
            let dfa_to: StateId = *state_map.entry(reachabe_states.clone()).or_insert_with(|| {
                state_stack.push(reachabe_states);
                dfa.add_state()
            });

            dfa._add_transition(dfa_from, dfa_to, Transition::IsNot(chars));
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
    pub fn run(&self, word: &str) -> bool {
        if self.states.is_empty() {
            return false;
        }

        let mut active_states = Set::new();
        active_states.insert(self.start.unwrap_or(StateId(0)));

        for c in word.chars() {
            let mut new_states = Set::new();
            for current_state in &active_states {
                for s in self[*current_state]
                    .transtions
                    .iter()
                    .filter(|(transition, _)| transition.allows(c))
                    .map(|(_, to)| to)
                    .copied()
                {
                    new_states.insert(s);
                }
            }

            if new_states.is_empty() {
                return false;
            }

            active_states = new_states;
        }

        active_states.iter().any(|s| self.final_states.contains(s))
    }
}
