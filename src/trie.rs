use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use crate::automaton::{Automaton, Deterministic};

#[derive(Debug)]
pub struct Trie(Automaton<Deterministic>);

impl Trie {
    pub fn load_from_file(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut words: Vec<_> = reader.lines().map_while(Result::ok).collect();
        words.sort();

        let automaton =
            Automaton::create_trie(&words.iter().map(String::as_str).collect::<Vec<_>>());

        Ok(Self(automaton))
    }

    pub fn new(words: &[&str]) -> Self {
        Self(Automaton::create_trie(words))
    }

    pub fn constains(&self, word: &str) -> bool {
        self.0.recognizes(word)
    }

    pub fn get_automaton(&self) -> &Automaton<Deterministic> {
        &self.0
    }
}

#[cfg(test)]
mod test {

    use crate::trie::Trie;

    #[test]
    fn test_trie() {
        let words: Vec<&str> = vec!["asd", "bin", "bing", "bong"];
        let trie = Trie::new(&words);

        assert!(trie.constains("bing"));
        assert!(trie.constains("bong"));
        assert!(trie.constains("bin"));
        assert!(trie.constains("asd"));
        assert!(!trie.constains("asdf"));
        assert!(!trie.constains("bi"));
        assert!(!trie.constains(""));
    }
}
