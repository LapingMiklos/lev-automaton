use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Deref,
    path::Path,
};

use crate::automaton::{Automaton, Deterministic};

#[derive(Debug)]
pub struct Trie(Automaton<Deterministic>);

impl Deref for Trie {
    type Target = Automaton<Deterministic>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Trie {
    pub fn load_from_file(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let words: Vec<_> = reader.lines().map_while(Result::ok).collect();

        let automaton = Automaton::create_trie(&words);

        Ok(Self(automaton))
    }

    pub fn new(words: &[String]) -> Self {
        Self(Automaton::create_trie(words))
    }
}

#[cfg(test)]
mod test {
    use crate::trie::Trie;

    #[test]
    fn test_trie() {
        let words: Vec<String> = vec!["asd", "bin", "bing", "bong"]
            .into_iter()
            .map(String::from)
            .collect();
        let trie = Trie::new(&words);

        assert!(trie.run("bing"));
        assert!(trie.run("bong"));
        assert!(trie.run("bin"));
        assert!(trie.run("asd"));
        assert!(!trie.run("asdf"));
        assert!(!trie.run("bi"));
        assert!(!trie.run(""));
    }
}
