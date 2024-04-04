use std::collections::HashMap;
// use std::sync::RwLock;
// use rayon::prelude::*;

#[derive(Debug)]
pub struct Dictionary<'a> {
    pub words: Vec<&'a str>,
    pub word_buckets: HashMap<u8, Vec<&'a str>>,
}

impl<'a> Dictionary<'a> {
    pub fn new(words: Vec<&'a str>) -> Self {
        let mut word_buckets = HashMap::new();
        for word in &words {
            word_buckets
                .entry(word.len() as u8)
                .and_modify(|vector: &mut Vec<&'a str>| vector.push(word))
                .or_insert(vec![word]);
        }

        Self {
            words,
            word_buckets,
        }
    }

    // TODO: use bloom filter for efficient checks of 100% invalid words?
    pub fn is_valid_word(&self, word: &str) -> bool {
        self.words.binary_search(&word).is_ok()
    }

    pub fn is_valid_prefix(&self, prefix: &str) -> bool {
        self.words
            .binary_search_by(|&word| {
                if word.starts_with(prefix) {
                    std::cmp::Ordering::Equal
                } else {
                    word.cmp(prefix)
                }
            })
            .is_ok()
    }

    pub fn get_values_from_range(&self, range: core::ops::Range<u8>) -> Vec<&Vec<&str>> {
        range
            .filter_map(|key| self.word_buckets.get(&key))
            .collect()
    }
}
