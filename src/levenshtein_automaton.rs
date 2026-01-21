use crate::automaton::{Automaton, Deterministic, NonDeterministic, StateId, Transition};

#[derive(Debug)]
pub struct LevenshteinAutomaton<T>(Automaton<T>);

impl<T> LevenshteinAutomaton<T> {
    pub fn get_automaton(&self) -> &Automaton<T> {
        &self.0
    }
}

impl LevenshteinAutomaton<NonDeterministic> {
    #[must_use]
    pub fn new(word: &str, k: usize) -> Self {
        let mut automaton: Automaton<NonDeterministic> = Automaton::default();

        let word_len = word.chars().count();
        let states: Vec<Vec<StateId>> = (0..=word_len)
            .map(|_| (0..=k).map(|_| automaton.add_state()).collect())
            .collect();

        for (i, c) in word.chars().enumerate() {
            for e in 0..=k {
                automaton.add_transition(states[i][e], states[i + 1][e], Transition::Is(c));
                if e < k {
                    automaton.add_transition(states[i][e], states[i][e + 1], Transition::Star);
                    automaton.add_transition(
                        states[i][e],
                        states[i + 1][e + 1],
                        Transition::Epsilon,
                    );
                    automaton.add_transition(states[i][e], states[i + 1][e + 1], Transition::Star);
                }
            }
        }

        for e in 0..=k {
            if e < k {
                automaton.add_transition(
                    states[word_len][e],
                    states[word_len][e + 1],
                    Transition::Star,
                );
            }
            automaton.make_state_final(states[word_len][e]);
        }

        Self(automaton)
    }
}

impl From<LevenshteinAutomaton<NonDeterministic>> for LevenshteinAutomaton<Deterministic> {
    fn from(nfa: LevenshteinAutomaton<NonDeterministic>) -> Self {
        Self(nfa.0.into())
    }
}

#[cfg(test)]
mod test {
    use crate::{automaton::Deterministic, levenshtein_automaton::LevenshteinAutomaton};

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

    const FOOD_LEV_4: [&str; 6] = ["", "****", "f***.", "***.d", "***d.", "**o*."];

    #[test]
    fn test_0th_degree_lev_automata() {
        let lev_aut = LevenshteinAutomaton::new(FOOD, 0);

        assert!(lev_aut.0.recognizes(FOOD));

        for word in FOOD_LEV_1 {
            assert!(!lev_aut.0.recognizes(word))
        }
    }

    #[test]
    fn test_1st_degree_lev_automata() {
        let lev_aut = LevenshteinAutomaton::new(FOOD, 1);

        assert!(lev_aut.0.recognizes(FOOD));

        for word in FOOD_LEV_1 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_2 {
            assert!(!lev_aut.0.recognizes(word))
        }
    }

    #[test]
    fn test_2nd_degree_lev_automata() {
        let lev_aut = LevenshteinAutomaton::new(FOOD, 2);

        assert!(lev_aut.0.recognizes(FOOD));

        for word in FOOD_LEV_1 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_2 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_3 {
            assert!(!lev_aut.0.recognizes(word))
        }
    }

    #[test]
    fn test_3rd_degree_lev_automata() {
        let lev_aut = LevenshteinAutomaton::new(FOOD, 3);

        assert!(lev_aut.0.recognizes(FOOD));

        for word in FOOD_LEV_1 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_2 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_3 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_4 {
            assert!(!lev_aut.0.recognizes(word))
        }
    }

    #[test]
    fn test_0th_degree_det_lev_automata() {
        let lev_aut: LevenshteinAutomaton<Deterministic> =
            LevenshteinAutomaton::new(FOOD, 0).into();

        assert!(lev_aut.0.recognizes(FOOD));

        for word in FOOD_LEV_1 {
            assert!(!lev_aut.0.recognizes(word))
        }
    }

    #[test]
    fn test_1st_degree_det_lev_automata() {
        let lev_aut: LevenshteinAutomaton<Deterministic> =
            LevenshteinAutomaton::new(FOOD, 1).into();

        assert!(lev_aut.0.recognizes(FOOD));

        for word in FOOD_LEV_1 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_2 {
            assert!(!lev_aut.0.recognizes(word))
        }
    }

    #[test]
    fn test_2nd_degree_det_lev_automata() {
        let lev_aut: LevenshteinAutomaton<Deterministic> =
            LevenshteinAutomaton::new(FOOD, 2).into();

        assert!(lev_aut.0.recognizes(FOOD));

        for word in FOOD_LEV_1 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_2 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_3 {
            assert!(!lev_aut.0.recognizes(word))
        }
    }

    #[test]
    fn test_3rd_degree_det_lev_automata() {
        let lev_aut: LevenshteinAutomaton<Deterministic> =
            LevenshteinAutomaton::new(FOOD, 3).into();

        assert!(lev_aut.0.recognizes(FOOD));

        for word in FOOD_LEV_1 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_2 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_3 {
            assert!(lev_aut.0.recognizes(word))
        }

        for word in FOOD_LEV_4 {
            assert!(!lev_aut.0.recognizes(word))
        }
    }
}
