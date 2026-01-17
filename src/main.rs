use std::{
    env,
    io::{self, BufRead},
    path::Path,
};

use crate::{automaton::Deterministic, levenshtein_automaton::LevenshteinAutomaton, trie::Trie};
use colored::Colorize;

pub mod automaton;
pub mod levenshtein_automaton;
pub mod trie;

fn main() {
    let path = env::var("LEV_SPELL_CHECK_DICT_PATH").unwrap_or("/usr/share/dict/words".into());

    let stdin = io::stdin();
    let reader = stdin.lock();

    let trie = Trie::load_from_file(Path::new(&path))
        .expect(&format!("Unable to open dictionary file: {path}"));

    for line in reader.lines() {
        let line = line.expect("STDIN FAIL");

        _ = line
            .split_whitespace()
            .map(|word| {
                if !trie.run(word) {
                    let aut = LevenshteinAutomaton::new(word, 1);
                    let aut: LevenshteinAutomaton<Deterministic> = aut.into();
                    let possible_corrections = aut.intersect(&trie);

                    print!("{}", word.red().strikethrough());
                    match possible_corrections.len() {
                        0 => {}
                        1 => {
                            print!(" -> {} ", possible_corrections[0].green().italic())
                        }
                        _ => {
                            print!(" -> {{ ");
                            for (i, correction) in possible_corrections.iter().enumerate() {
                                print!("{}", correction.green());
                                if i != possible_corrections.len() - 1 {
                                    print!(", ")
                                }
                            }
                            print!(" }} ");
                        }
                    }
                } else {
                    print!("{word} ")
                }
            })
            .collect::<Vec<_>>();
        println!();
    }
}
