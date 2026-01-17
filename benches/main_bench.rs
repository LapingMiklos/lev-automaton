use std::{env, fs::File, path::Path};

use criterion::{Criterion, criterion_group, criterion_main};
use lev_automaton::{automaton::Deterministic, levenshtein_automaton::LevenshteinAutomaton, trie::Trie};

fn lev_automaton_bench(c: &mut Criterion) {
    let mut words_file = File::open("test_data/words.json").unwrap();
    let words: Vec<(String, String)> = serde_json::from_reader(&mut words_file).unwrap();

    let path = env::var("LEV_SPELL_CHECK_DICT_PATH").unwrap_or("/usr/share/dict/words".into());
    let trie = Trie::load_from_file(Path::new(&path))
        .unwrap_or_else(|_| panic!("Unable to open dictionary file: {path}"));

    let mut group = c.benchmark_group("Levenshtein Automaton");
    group.bench_function("1st degree", |b| {
        b.iter(|| {
            for (misspelled, _) in words.iter() {
                let aut = LevenshteinAutomaton::new(misspelled, 1);
                let aut: LevenshteinAutomaton<Deterministic> = aut.into();
                let _ = aut.intersect(&trie);
            }
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = lev_automaton_bench
}

criterion_main!(benches);
