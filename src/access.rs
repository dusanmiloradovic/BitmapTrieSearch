use crate::trie::{Trie, TrieSearchResult};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

pub struct TrieMap {
    map: HashMap<String, Arc<RwLock<Trie>>>,
}

impl TrieMap {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn add_word(&mut self, trie_id: &str, word: &str, dictionary_index: u32, dictionary_attribute: u8, position: u16) {
        let trie = self.map
            .entry(trie_id.to_string())
            .or_insert_with(|| Arc::new(RwLock::new(Trie::new())));
        
        let mut trie_lock = trie.write().unwrap();
        trie_lock.add_word(word, dictionary_index, dictionary_attribute,position);
    }

    pub fn search(&self, trie_id: &str, word: &str) -> Option<Vec<TrieSearchResult>> {
        self.map.get(trie_id).map(|trie| {
            let trie_lock = trie.read().unwrap();
            trie_lock.search(word)
        })
    }
}