use crate::nfa_lev::NfaLev;

pub mod automaton;
pub mod nfa_lev;

fn main() {
    let nfa = NfaLev::new("food", 1);
    println!("{nfa:?}");

    println!("{:?}", nfa.run("brod"))
}
