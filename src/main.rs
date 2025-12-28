use crate::{automaton::Deterministic, levenshtein_automaton::LevenshteinAutomaton};

pub mod automaton;
pub mod levenshtein_automaton;
pub mod trie;

fn main() {
    let nfa = LevenshteinAutomaton::new("food", 0);
    dbg!(&nfa);
    println!("{:?}", nfa.run("brod"));

    let dfa: LevenshteinAutomaton<Deterministic> = nfa.into();
    dbg!(dfa);
}
