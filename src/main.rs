use std::path::Path;

use crate::{automaton::Deterministic, levenshtein_automaton::LevenshteinAutomaton, trie::Trie};

pub mod automaton;
pub mod levenshtein_automaton;
pub mod trie;

fn main() {
    let nfa = LevenshteinAutomaton::new("donkep", 1);
    // dbg!(&nfa);
    println!("{:?}", nfa.run("brod"));

    let dfa: LevenshteinAutomaton<Deterministic> = nfa.into();
    // dbg!(&dfa);

    let trie = Trie::load_from_file(Path::new("/usr/share/dict/words")).expect("asd");

    dbg!(trie.run("donkey"));

    dbg!(dfa.intersect(&trie));
}
