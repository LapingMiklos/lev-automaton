use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;

use crate::automaton::{Automaton, Deterministic, StateId, Transition};

#[derive(Debug, Clone)]
pub struct Trie(Automaton<Deterministic>);

impl Trie {
    pub fn load_from_file(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let words: Vec<_> = reader.lines().map_while(Result::ok).collect();
        Ok(Self::new(
            &mut words.iter().map(String::as_str).collect::<Vec<_>>(),
        ))
    }

    pub fn new(words: &mut [&str]) -> Self {
        words.sort();
        let mut automaton = Automaton::default();
        let start_state = automaton.add_state();

        Self::add_trie_states(&mut automaton, start_state, words);

        Self(automaton)
    }

    fn add_trie_states(
        automaton: &mut Automaton<Deterministic>,
        start_state: StateId,
        words: &[&str],
    ) {
        _ = words
            .iter()
            .chunk_by(|w| w.chars().next())
            .into_iter()
            .filter_map(|(char, word_group)| char.map(|c| (c, word_group)))
            .map(|(char, word_group)| {
                let new_state = automaton.add_state();
                let transition_added =
                    automaton.add_transition(start_state, new_state, Transition::Is(char));
                debug_assert!(transition_added);
                let suffixes: Vec<&str> = word_group.map(|w| suffix_of(w)).collect();
                if suffixes.iter().any(|w| w.is_empty()) {
                    automaton.make_state_final(new_state);
                }

                Self::add_trie_states(automaton, new_state, &suffixes);
            })
            .collect::<Vec<_>>();
    }

    pub fn contains(&self, word: &str) -> bool {
        self.0.recognizes(word)
    }

    pub fn filter(&self, automata: &Automaton<Deterministic>) -> Vec<String> {
        automata.intersect(&self.0)
    }
}

fn suffix_of(word: &str) -> &str {
    match word.char_indices().nth(1) {
        Some((idx, _)) => &word[idx..],
        None => "",
    }
}

#[cfg(test)]
mod test {

    use crate::trie::Trie;

    #[test]
    fn test_trie() {
        let mut words: Vec<&str> = vec!["asd", "bin", "bing", "bong"];
        let trie = Trie::new(words.as_mut_slice());

        assert!(trie.contains("bing"));
        assert!(trie.contains("bong"));
        assert!(trie.contains("bin"));
        assert!(trie.contains("asd"));
        assert!(!trie.contains("asdf"));
        assert!(!trie.contains("bi"));
        assert!(!trie.contains(""));
    }
}
