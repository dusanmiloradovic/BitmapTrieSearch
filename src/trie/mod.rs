pub mod entry;
#[cfg(test)]
mod test;

pub use self::entry::{
    MAX_DIRECT_ENTRIES, NodeIndex, TrieEntry, TrieEntryG, TrieEntryOp, TrieEntryV,
};
use crate::encoding::idx;
use std::collections::HashMap;

/*
This is the Trie implementation for contextual search.
The primary use case is autocomplete search in context.
Each Trie is registered for one type of data. The structure of data is not enforced, but its assumed
that it can be decomposed into attributes. For example, if a backend structure is JSON {Name, Email, Phone, Address},
the client decides which attributes to make searchable , and put them in Trie.
We store a record with the actual data in a dictionary entry.
Since each word can have multiple dictionary entries (for example, two users named John Doe, both pointing
to different dictionary entry), each terminated word in TrieEntry can point to multiple dictionaries.
To make trie implementation efficient, we need to make the leaf node simple and implement Copy trait,
so the dictionary will be indirect.
In node index we store the address of dictionary entries, and from there we do the lookup.
 */

const MAX_SEARCH_RESULTS: usize = 10;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DictionaryMapEntry {
    pub entries: Vec<(u32, u8)>,
    // each terminated word in trie maps to one dictionary entry and one attribute (if no attribute, use default attribute 0)
    // attribute is expected as u8, the dictionary itself should keep the mapping of attributes(if there is one)
}

#[derive(Debug)]
pub struct TrieSearchResult {
    pub word: String,
    pub entries: DictionaryMapEntry,
}

// TODO trieentry removal from the Trie
// Since updating the indices after the deletion is costly
// we will not physically remove the entry from vector
// instead we will create a list of free row numbers, and push it there
// when the trie is expanded, first it will check if there are free rows available
// and then only expand the trie vector

/*
Deletion: we will always from Trie by the whole dictionary entry
First we will search the dictionary for exact match, delete it and find related dictionary map entry
Then, we remove that index from entries vector
If the entries vector is empty after that, we get the leaf node from the dictionary map entry
and then traverse up to the top to remove the node from trie
 */

#[derive(Debug)]
pub struct Trie {
    trie_entries: Vec<TrieEntry>,
    dictionary_map: HashMap<usize, DictionaryMapEntry>, //One NodeIndex to many DictionaryEntries (+ attribute)
    free_list: Vec<usize>,
}

// dictionary will keep the map of attributes, the

impl Trie {
    pub fn new() -> Self {
        let mut t = Trie {
            trie_entries: Vec::new(),
            dictionary_map: HashMap::new(),
            free_list: Vec::new(),
        };
        let v = vec![(
            0,
            NodeIndex {
                index: 0,
                terminated: false,
            },
        )]; //root node
        let tt = TrieEntryV(v);
        t.trie_entries.push(TrieEntry::TrieEntryV(tt));
        t
    }

    fn update_dictionary_entry(
        &mut self,
        curr_row: usize,
        dictionary_index: u32,
        dictionary_attribute: u8,
    ) {
        let v = self.dictionary_map.get_mut(&curr_row);
        match v {
            Some(e) => {
                let v = &mut e.entries;
                for vv in v.iter_mut() {
                    if vv.0 == dictionary_index && vv.1 == dictionary_attribute {
                        return;
                    }
                }
                v.push((dictionary_index, dictionary_attribute));
            }
            None => {
                let e = DictionaryMapEntry {
                    entries: vec![(dictionary_index, dictionary_attribute)],
                };
                self.dictionary_map.insert(curr_row, e);
            }
        }
    }

    fn remove_dictionary_entry(
        &mut self,
        curr_row: usize,
        dictionary_index: u32,
        dictionary_attribute: u8,
    ) -> bool {
        if let Some(e) = self.dictionary_map.get_mut(&curr_row) {
            let v = &mut e.entries;
            for (i, vv) in v.iter().enumerate() {
                if vv.0 == dictionary_index && vv.1 == dictionary_attribute {
                    v.remove(i);
                    break;
                }
            }
            if v.len() == 0 {
                self.dictionary_map.remove(&curr_row);
                true // if the dictionary map is empty, we can remove the trie entry(no trie leaf points to it)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn add_word(&mut self, word: &str, dictionary_index: u32, dictionary_attribute: u8) {
        let mut curr_row = 0;
        let mut prev_row = 0;
        let mut should_add = false;
        let mut prev_c: char = 0 as char;
        // once we add the new entry, all the leaf nodes will have to be created anew
        let word_len = word.chars().count();
        for (i, c) in word.chars().enumerate() {
            let terminated = i == word_len - 1;
            if should_add {
                let v = vec![(
                    idx(c),
                    NodeIndex {
                        index: 0,
                        terminated,
                    },
                )];
                let tt = TrieEntryV(v);
                self.trie_entries.push(TrieEntry::TrieEntryV(tt));
                let position = self.trie_entries.len() as u32 - 1;
                self.trie_entries[prev_row].update_index(prev_c, position);
                prev_c = c;
                prev_row = position as usize;
                curr_row = self.trie_entries.len() - 1;
                continue;
            }
            prev_c = c;
            prev_row = curr_row;
            let entry = &mut self.trie_entries[curr_row];
            let existing = entry.find(c);

            if let Some(node) = existing {
                if terminated {
                    entry.update_terminated(c, true);
                }
                if node.index != 0 {
                    prev_row = curr_row;
                    curr_row = node.index as usize;
                    continue;
                } else {
                    should_add = true;
                }
            } else {
                should_add = true;
                let ni = NodeIndex {
                    index: 0,
                    terminated,
                };
                entry.add(c, ni);
                if let TrieEntry::TrieEntryV(v) = entry {
                    if v.0.len() >= MAX_DIRECT_ENTRIES {
                        let promoted = TrieEntryG::promote(v);
                        self.trie_entries[curr_row] = TrieEntry::TrieEntryG(promoted);
                    }
                }
            }
        }
        self.update_dictionary_entry(prev_row, dictionary_index, dictionary_attribute);
    }
    pub fn search(&self, term: &str) -> Vec<TrieSearchResult> {
        let mut res = Vec::new();
        let mut curr_row = 0;
        let mut last_terminated = false;
        let mut prev_row = 0;
        // find if the whole
        for c in term.chars() {
            prev_row = curr_row;
            if let Some(ni) = self.trie_entries[curr_row].find(c) {
                curr_row = ni.index as usize;
                last_terminated = ni.terminated;
            } else {
                return res;
            }
            // if any word was found it will be in the return vector, from here return all the children (filtered with terminated)
        }
        if last_terminated {
            if let Some(entries) = self.dictionary_map.get(&prev_row) {
                res.push(TrieSearchResult {
                    word: term.to_string(),
                    entries: entries.clone(),
                });
            }
            if curr_row == 0 {
                return res; //last entry
            }
        }
        let entry = &self.trie_entries[curr_row];
        let children = entry.get_all();
        let mut bfs_stack: Vec<(String, NodeIndex)> = Vec::new();
        for (c, ni) in children {
            let w = term.to_string() + &c.to_string();
            bfs_stack.push((w, ni));
        }
        while bfs_stack.len() > 0 && res.len() < MAX_SEARCH_RESULTS {
            let e = bfs_stack.pop();
            match e {
                None => break,
                Some((w, ni)) => {
                    if ni.terminated {
                        if let Some(entries) = self.dictionary_map.get(&curr_row) {
                            res.push(TrieSearchResult {
                                word: w.clone(),
                                entries: entries.clone(),
                            });
                        }
                    }

                    if ni.index != 0 {
                        let entry = &self.trie_entries[ni.index as usize];
                        let children = entry.get_all();
                        for (c, ni) in children {
                            bfs_stack.push((w.to_string() + &c.to_string(), ni));
                        }
                    }
                    curr_row = ni.index as usize;
                }
            }
        }
        res
    }

    pub fn delete_word(&mut self, word: &str, dictionary_index: u32, dictionary_attribute: u8) {
        let mut curr_row = 0;
        let mut prev_row = 0;
        let mut trail: Vec<(usize, char, bool)> = Vec::new();
        for c in word.chars() {
            prev_row = curr_row;
            if let Some(ni) = self.trie_entries[curr_row].find(c) {
                curr_row = ni.index as usize;
                trail.push((prev_row, c, ni.terminated));
            } else {
                return;
            }
        }

        let removed =
            self.remove_dictionary_entry(prev_row, dictionary_index, dictionary_attribute);
        if removed {
            let mut row_removed = true; //looping condition
            let mut cnt = 0;
            while row_removed && trail.len() > 0 {
                let (row, c, terminated) = trail.pop().unwrap();
                if cnt == 0 {
                    self.trie_entries[row].update_terminated(c, false);
                    self.free_list.push(row);
                } else if terminated {
                    break;
                }

                let ni = self.trie_entries[row].find(c).unwrap();
                if cnt != 0 {
                    self.trie_entries[row].update_index(c, 0);
                }
                if ni.index == 0 || cnt != 0 {
                    // no descedants left, we can remove this character from trie entry
                    let all_removed = self.trie_entries[row].remove(c);
                    // TODO if all removed, put this row number into free list
                    if all_removed {
                        self.free_list.push(row);
                    } else {
                        row_removed = false;
                    }
                } else {
                    row_removed = false;
                }
                cnt += 1;
            }
        }
    }
}
