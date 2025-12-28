use crate::levenshtein_automata::{DfaLev, NfaLev};

pub mod automaton;
pub mod levenshtein_automata;

fn main() {
    let nfa = NfaLev::new("food", 0);
    dbg!(&nfa);
    println!("{:?}", nfa.run("brod"));

    let dfa: DfaLev = nfa.into();
    dbg!(dfa);
}
