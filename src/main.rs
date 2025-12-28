use crate::levenshtein_automaton::{DfaLev, NfaLev};

pub mod automaton;
pub mod levenshtein_automaton;

fn main() {
    let nfa = NfaLev::new("food", 0);
    dbg!(&nfa);
    println!("{:?}", nfa.run("brod"));

    let dfa: DfaLev = nfa.into();
    dbg!(dfa);
}
