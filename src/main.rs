use std::{
    env,
    io::{self, BufRead},
    path::Path,
};

use colored::Colorize;
use lev_automaton::{
    automaton::Deterministic, levenshtein_automaton::LevenshteinAutomaton,
    spell_checker::SpellChecker, trie::Trie,
};

fn main() {
    let path = env::var("LEV_SPELL_CHECK_DICT_PATH").unwrap_or("/usr/share/dict/words".into());
    let trie = Trie::load_from_file(Path::new(&path))
        .unwrap_or_else(|_| panic!("Unable to open dictionary file: {path}"));
    let spell_checker = SpellChecker::new(trie, |word, trie| {
        let aut = LevenshteinAutomaton::new(word, 1);
        let aut: LevenshteinAutomaton<Deterministic> = aut.into();
        trie.filter(aut.get_automaton())
    });

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("STDIN FAIL");

        _ = line
            .split_whitespace()
            .map(|word| match spell_checker.check_word(word) {
                Ok(()) => {
                    print!("{word} ")
                }
                Err(possible_corrections) => {
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
                }
            })
            .collect::<Vec<_>>();
        println!();
    }
}
