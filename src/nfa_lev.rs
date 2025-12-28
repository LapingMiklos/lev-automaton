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

#[cfg(test)]
mod test {
    use crate::nfa_lev::NfaLev;

    const FOOD: &str = "food";

    const FOOD_LEV_1: [&str; 12] = [
        "foo", "foo.d", "food.", "fo*d", "foo*", "fo.od", "fod", "f*od", "ood", ".food", "f.ood",
        "*ood",
    ];

    const FOOD_LEV_2: [&str; 50] = [
        "oo*", "f*od.", "f.o*d", "fo.od.", "f*.od", "fo*", "f..ood", "*oo.d", "*.ood", "fod.",
        ".fod", "foo.d.", ".f*od", "*o*d", "fo.o.d", ".foo.d", "..food", "oo", "fo*d.", "o*d",
        "*oo*", "fo.*d", ".fo*d", "fd", "f.oo.d", ".food.", "*od", "*ood.", "food..", "**od",
        ".f.ood", "fo*.d", "f*o", "*oo", "f*o*", "fo.o*", "o.od", "f**d", ".foo", "fo..od",
        "f*o.d", "*o.od", "od", ".foo*", "oo.d", "f.oo", "f.o.od", "fo.o", "f.*od", "fo",
    ];

    const FOOD_LEV_3: [&str; 10] = [
        "*o", "f*.", "f.o*d.", "f..od.", "f*.o", "fo**.", "f...ood", "*.d", "f", "o",
    ];

    #[test]
    fn test_0th_degree_lev_autamata() {
        let target = "food";
        let lev_aut = NfaLev::new(target, 0);

        assert!(lev_aut.run(target));

        for word in FOOD_LEV_1 {
            assert!(!lev_aut.run(word))
        }
    }

    #[test]
    fn test_1st_degree_lev_autamata() {
        let lev_aut = NfaLev::new(FOOD, 1);

        assert!(lev_aut.run(FOOD));

        for word in FOOD_LEV_1 {
            assert!(lev_aut.run(word))
        }

        for word in FOOD_LEV_2 {
            assert!(!lev_aut.run(word))
        }
    }

    #[test]
    fn test_2nd_degree_lev_autamata() {
        let lev_aut = NfaLev::new(FOOD, 2);

        assert!(lev_aut.run(FOOD));

        for word in FOOD_LEV_1 {
            assert!(lev_aut.run(word))
        }

        for word in FOOD_LEV_2 {
            assert!(lev_aut.run(word))
        }

        for word in FOOD_LEV_3 {
            assert!(!lev_aut.run(word))
        }
    }
}
