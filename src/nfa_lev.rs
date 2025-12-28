use std::{collections::HashMap, ops::Deref};

use crate::automaton::{Automaton, NonDeterministic, StateId, Transition};

#[derive(Debug)]
pub struct NfaLev(Automaton<NonDeterministic>);

impl Deref for NfaLev {
    type Target = Automaton<NonDeterministic>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NfaLev {
    #[must_use]
    pub fn new(word: &str, k: usize) -> Self {
        let mut automaton = Automaton::default();

        let mut states: HashMap<(usize, usize), StateId> = HashMap::new();
        let word_len = word.chars().count();
        for i in 0..=word_len {
            for e in 0..=k {
                states.insert((i, e), automaton.add_state());
            }
        }

        for (i, c) in word.chars().enumerate() {
            for e in 0..=k {
                automaton.add_transition(states[&(i, e)], states[&(i + 1, e)], Transition::Is(c));
                if e < k {
                    automaton.add_transition(
                        states[&(i, e)],
                        states[&(i, e + 1)],
                        Transition::Star,
                    );
                    automaton.add_transition(
                        states[&(i, e)],
                        states[&(i + 1, e + 1)],
                        Transition::Epsilon,
                    );
                    automaton.add_transition(
                        states[&(i, e)],
                        states[&(i + 1, e + 1)],
                        Transition::Star,
                    );
                }
            }
        }

        for e in 0..=k {
            if e < k {
                automaton.add_transition(
                    states[&(word_len, e)],
                    states[&(word_len, e + 1)],
                    Transition::Star,
                );
            }
            automaton.make_state_final(states[&(word_len, e)]);
        }

        Self(automaton)
    }
}
