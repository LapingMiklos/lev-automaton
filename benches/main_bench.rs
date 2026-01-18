use std::{env, fs::File, path::Path};

use criterion::{Criterion, criterion_group, criterion_main};
use lev_automaton::{
    automaton::Deterministic, levenshtein_automaton::LevenshteinAutomaton,
    spell_checker::SpellChecker, trie::Trie,
};

fn lev_automaton_bench(c: &mut Criterion) {
    let mut words_file = File::open("test_data/words.json").unwrap();
    let words: Vec<(String, String)> = serde_json::from_reader(&mut words_file).unwrap();

    let path = env::var("LEV_SPELL_CHECK_DICT_PATH").unwrap_or("/usr/share/dict/words".into());
    let trie = Trie::load_from_file(Path::new(&path))
        .unwrap_or_else(|_| panic!("Unable to open dictionary file: {path}"));
    let mut group = c.benchmark_group("Levenshtein Automaton");

    for degree in 1..=2 {
        let spell_checker = SpellChecker::new(trie.clone(), |word, trie| {
            let aut = LevenshteinAutomaton::new(word, degree);
            let aut: LevenshteinAutomaton<Deterministic> = aut.into();
            aut.get_automaton().intersect(&trie.get_automaton())
        });

        group.bench_function(format!("degree: {degree}"), |b| {
            b.iter(|| {
                for (misspelled, _) in words.iter() {
                    let _ = spell_checker.check_word(misspelled);
                }
            });
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = lev_automaton_bench
}

criterion_main!(benches);
