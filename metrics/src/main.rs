use std::{env, fs::File, path::Path};

use lev_automaton::{
    automaton::Deterministic, levenshtein_automaton::LevenshteinAutomaton,
    spell_checker::SpellChecker, trie::Trie,
};

fn main() {
    let path = env::var("LEV_SPELL_CHECK_DICT_PATH").unwrap_or("/usr/share/dict/words".into());
    let trie = Trie::load_from_file(Path::new(&path))
        .unwrap_or_else(|_| panic!("Unable to open dictionary file: {path}"));

    let mut words_file = File::open("test_data/words.json").unwrap();
    let words: Vec<(String, String)> = serde_json::from_reader(&mut words_file).unwrap();
    let words: Vec<_> = words
        .into_iter()
        .filter(|(_, correct)| trie.constains(correct))
        .collect();

    for degree in 1..=3 {
        let spell_checker = SpellChecker::new(trie.clone(), |word, trie| {
            let aut = LevenshteinAutomaton::new(word, degree);
            let aut: LevenshteinAutomaton<Deterministic> = aut.into();
            aut.get_automaton().intersect(trie.get_automaton())
        });

        let mut unambiguous_corrections = 0;
        let mut ambiguous_corrections = 0;
        let mut not_corrected = 0;
        for (misspelled, correct) in words.iter() {
            let res = spell_checker.check_word(misspelled);
            match res {
                Ok(()) => {}
                Err(corrections) => {
                    if corrections.iter().any(|c| c == correct) {
                        if corrections.len() == 1 {
                            unambiguous_corrections += 1;
                        } else {
                            ambiguous_corrections += 1;
                        }
                    } else {
                        not_corrected += 1;
                    }
                }
            }
        }

        println!("Levenshtein automaton of degree: {degree}");
        println!("Word count:               {}", words.len());
        println!(
            "Unambiguous corrections:  {} ({:.2}%)",
            unambiguous_corrections,
            unambiguous_corrections as f64 / words.len() as f64 * 100.0
        );
        println!(
            "Ambiguous corrections:    {} ({:.2}%)",
            ambiguous_corrections,
            ambiguous_corrections as f64 / words.len() as f64 * 100.0
        );
        println!(
            "Not corrected:            {} ({:.2}%)",
            not_corrected,
            not_corrected as f64 / words.len() as f64 * 100.0
        );
        println!();
    }
}
