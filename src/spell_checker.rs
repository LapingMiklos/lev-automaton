use crate::trie::Trie;

pub struct SpellChecker<F>
where
    F: Fn(&str, &Trie) -> Vec<String>,
{
    trie: Trie,
    correction_func: F,
}

impl<F> SpellChecker<F>
where
    F: Fn(&str, &Trie) -> Vec<String>,
{
    pub fn new(trie: Trie, correction_func: F) -> Self {
        Self {
            trie,
            correction_func,
        }
    }

    pub fn check_word(&self, word: &str) -> Result<(), Vec<String>> {
        if self.trie.constains(word) {
            Ok(())
        } else {
            Err((self.correction_func)(word, &self.trie))
        }
    }
}
