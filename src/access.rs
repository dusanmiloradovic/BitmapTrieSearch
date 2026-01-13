use crate::trie::{Trie, TrieSearchResult};

pub struct TrieMap{
    map: std::collections::HashMap<String, Trie>
}

impl TrieMap{
    pub fn new() -> Self{
        Self{map: std::collections::HashMap::new()}
    }
    pub fn add_word(&mut self, trieId: &str, word: &str){
        self.map.entry(trieId.to_string()).or_insert(Trie::new()).add_word(word);
    }

    pub fn search(&self, trieId: &str, word: &str) -> Option<Vec<TrieSearchResult>>{
        match self.map.get(trieId){
            Some(trie) => Some(trie.search(word)),
            None => None
        }
    }
}