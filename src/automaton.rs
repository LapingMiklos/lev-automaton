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

    pub fn merge(&self, other: &Self) -> Option<char> {
        match (self, other) {
            (Self::Is(c1), Self::Is(c2)) => (c1 == c2).then_some(*c1),
            (Self::IsNot(cs), Self::Is(c)) => (!cs.contains(c)).then_some(*c),
            (Self::Is(c), Self::IsNot(cs)) => (!cs.contains(c)).then_some(*c),
            (Self::Star, Self::Is(c)) => Some(*c),
            (Self::Is(c), Self::Star) => Some(*c),
            _ => None,
        }
    }

    pub fn have_overlap(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Is(c1), Self::Is(c2)) => c1 == c2,
            (Self::IsNot(cs), Self::Is(c)) => !cs.contains(c),
            (Self::Is(c), Self::IsNot(cs)) => !cs.contains(c),
            (Self::IsNot(_), Self::IsNot(_)) => true,
            (Self::Epsilon, _) => true,
            (_, Self::Epsilon) => true,
            (Self::Star, _) => true,
            (_, Self::Star) => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    transitions: Vec<(Transition, StateId)>,
}

impl State {
    const fn new() -> Self {
        Self {
            transitions: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum NonDeterministic {}

#[derive(Debug, Clone)]
pub enum Deterministic {}

#[derive(Debug, Clone)]
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
            .transitions
            .iter()
            .filter(move |(transition, _)| pred(transition))
            .map(|(_, to)| *to)
    }
}

impl Automaton<NonDeterministic> {
    pub fn add_transition(&mut self, from: StateId, to: StateId, transition: Transition) {
        self[from].transitions.push((transition, to));
    }

    pub fn recognizes(&self, word: &str) -> bool {
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
                        .transitions
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
                let reachable_states = nfa.eps_closure(reachable_states);
                let dfa_to: StateId =
                    *state_map
                        .entry(reachable_states.clone())
                        .or_insert_with(|| {
                            state_stack.push(reachable_states);
                            dfa.add_state()
                        });

                let added_transition = dfa.add_transition(dfa_from, dfa_to, Transition::Is(*c));
                debug_assert!(added_transition)
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

            let added_transition = dfa.add_transition(dfa_from, dfa_to, Transition::IsNot(chars));
            debug_assert!(added_transition)
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
    pub fn intersect(&self, other: &Self) -> Vec<String> {
        let mut words = vec![];
        let mut stack = vec![(
            String::new(),
            self.start.unwrap_or(StateId(0)),
            other.start.unwrap_or(StateId(0)),
        )];
        while let Some((word, self_state, other_state)) = stack.pop() {
            for (self_transition, new_self_state) in &self[self_state].transitions {
                for (other_transition, new_other_state) in &other[other_state].transitions {
                    if let Some(char) = self_transition.merge(other_transition) {
                        let mut new_word = word.clone();
                        new_word.push(char);

                        if self.final_states.contains(new_self_state)
                            && other.final_states.contains(new_other_state)
                        {
                            words.push(new_word.clone())
                        }
                        stack.push((new_word, *new_self_state, *new_other_state))
                    }
                }
            }
        }

        words
    }

    #[must_use]
    pub fn add_transition(&mut self, from: StateId, to: StateId, transition: Transition) -> bool {
        if self[from]
            .transitions
            .iter()
            .any(|(t, _)| t.have_overlap(&transition))
        {
            return false;
        }

        self[from].transitions.push((transition, to));
        true
    }

    pub fn recognizes(&self, word: &str) -> bool {
        if self.states.is_empty() {
            return false;
        }

        let mut active_state = self.start.unwrap_or(StateId(0));

        for c in word.chars() {
            let transitions: Vec<_> = self[active_state]
                .transitions
                .iter()
                .filter(|(transition, _)| transition.allows(c))
                .map(|(_, to)| to)
                .collect();

            if let Some(new_state) = transitions.into_iter().next() {
                active_state = *new_state
            } else {
                return false;
            }
        }

        self.final_states.contains(&active_state)
    }
}
