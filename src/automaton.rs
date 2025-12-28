use std::{
    collections::HashSet,
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

impl ToString for Transition {
    fn to_string(&self) -> String {
        match self {
            Self::Is(c) => c.to_string(),
            Self::Star => "*".into(),
            Self::Epsilon => "ε".into()
        }
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

#[derive(Debug, Default)]
pub struct Automaton {
    states: Vec<State>,
    start: Option<StateId>,
    final_states: HashSet<StateId>,
}

impl Index<StateId> for Automaton {
    type Output = State;

    fn index(&self, index: StateId) -> &Self::Output {
        &self.states[index.0]
    }
}

impl IndexMut<StateId> for Automaton {
    fn index_mut(&mut self, index: StateId) -> &mut Self::Output {
        &mut self.states[index.0]
    }
}

impl Automaton {
    pub fn add_state(&mut self) -> StateId {
        self.states.push(State::new());
        StateId(self.states.len() - 1)
    }

    pub fn add_transition(&mut self, from: StateId, to: StateId, transition: Transition) {
        self[from].transtions.push((transition, to));
    }

    pub fn make_state_final(&mut self, index: StateId) {
        self.final_states.insert(index);
    }

    pub const fn set_start_state(&mut self, index: StateId) {
        self.start = Some(index);
    }
}
