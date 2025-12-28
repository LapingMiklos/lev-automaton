use std::{
    collections::HashSet,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transition {
    Is(char),
    Star,
    Epsilon,
}

impl Transition {
    pub fn allows(&self, c: char) -> bool {
        match self {
            Self::Is(character) => c == *character,
            Self::Star => true,
            Self::Epsilon => false,
        }
    }

    pub fn is_epsilon(&self) -> bool {
        matches!(self, Self::Epsilon)
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
    final_states: HashSet<StateId>,
    _determinism_marker: PhantomData<T>,
}

impl<T> Default for Automaton<T> {
    fn default() -> Self {
        Self {
            states: vec![],
            start: None,
            final_states: HashSet::new(),
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
}

impl Automaton<NonDeterministic> {
    pub fn add_transition(&mut self, from: StateId, to: StateId, transition: Transition) {
        self[from].transtions.push((transition, to));
    }

    pub fn run(&self, word: &str) -> bool {
        if self.states.is_empty() {
            return false;
        }

        let mut active_states = HashSet::new();
        active_states.insert(self.start.unwrap_or(StateId(0)));

        for c in word.chars() {
            for state in &active_states.clone() {
                self.add_eps_states(&mut active_states, *state);
            }

            let mut new_states = HashSet::new();
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

            active_states = new_states;
        }

        for state in &active_states.clone() {
            self.add_eps_states(&mut active_states, *state);
        }

        active_states.iter().any(|s| self.final_states.contains(s))
    }

    fn add_eps_states(&self, new_states: &mut HashSet<StateId>, current_state: StateId) {
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
}
