use crate::encoding::{CHARS, idx};
use std::fmt;
#[derive(Clone, Debug, Copy)]
struct NodeIndex {
    index: u32,            //0 - leaf node
    dictionary_index: u32, // 0 for no index -> if terminated, this is a link to an actual dictionary entry
    terminated: bool,      // it can be terminated for one word, and still continue in the trie
}

const MAX_DIRECT_ENTRIES: usize = 5;

// TODO add types of dictionary entries
#[derive(Clone, Debug)]
struct TrieEntryG {
    bitmap: u64,
    positions: Vec<NodeIndex>,
}

struct TrieEntryV(Vec<(u8, NodeIndex)>);

impl fmt::Debug for TrieEntryV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Here we can choose to hide the actual data
        let r = &self.0;
        write!(f, "TrieEntryV: [")?;
        for (i, z) in r.iter().enumerate() {
            let c = CHARS.chars().nth(z.0 as usize).unwrap();
            write!(f, "({},{:?})", c, z.1)?;
        }
        write!(f, "]")
    }
}

trait TrieEntryOp {
    fn find(&self, c: char) -> Option<NodeIndex>;
    fn add(&mut self, c: char, ni: NodeIndex);
    fn update_index(&mut self, c: char, index: u32);
    fn update_terminated(&mut self, c: char, terminated: bool);
}

#[derive(Debug)]
enum TrieEntry {
    TrieEntryG(TrieEntryG),
    TrieEntryV(TrieEntryV),
}

impl TrieEntryOp for TrieEntry {
    fn find(&self, c: char) -> Option<NodeIndex> {
        let char_idx = idx(c);
        match self {
            TrieEntry::TrieEntryV(v) => {
                let m = &v.0;
                for vv in m.iter() {
                    if vv.0 == char_idx {
                        return Some(vv.1);
                    }
                }
                None
            }
            TrieEntry::TrieEntryG(v) => {
                // For TrieEntryG, if it is in the bitmap, it exists
                if (v.bitmap & (1u64 << char_idx)) != 0 {
                    v.get(char_idx).copied()
                } else {
                    None
                }
            }
        }
    }

    fn add(&mut self, c: char, ni: NodeIndex) {
        match self {
            TrieEntry::TrieEntryV(v) => v.0.push((idx(c), ni)),
            _ => {} // TODO
        }
    }

    fn update_index(&mut self, c: char, index: u32) {
        let ix = idx(c);
        match self {
            TrieEntry::TrieEntryV(v) => {
                for vv in v.0.iter_mut() {
                    if vv.0 == ix {
                        let mut ni = vv.1;
                        ni.index = index;
                        vv.1 = ni;
                    }
                }
            }
            _ => {}
        }
    }

    fn update_terminated(&mut self, c: char, terminated: bool) {
        // TODO add the optional dictionary index here
        let ix = idx(c);
        match self {
            TrieEntry::TrieEntryV(v) => {
                for vv in v.0.iter_mut() {
                    if vv.0 == ix {
                        let mut ni = vv.1;
                        ni.terminated = terminated;
                        vv.1 = ni;
                    }
                }
            }
            _ => {}
        }
    }
}

impl TrieEntryG {
    pub fn insert_at(&mut self, bit_pos: u8, node: NodeIndex) {
        let mask = (1u64 << bit_pos) - 1;
        // The index in the Vec is the number of set bits to the right of bit_pos
        let array_idx = (self.bitmap & mask).count_ones() as usize;

        if (self.bitmap & (1u64 << bit_pos)) == 0 {
            // Bit was not set: insert new entry into the vector
            self.bitmap |= 1u64 << bit_pos;
            self.positions.insert(array_idx, node);
        } else {
            // Bit was already set: update existing entry
            self.positions[array_idx] = node;
        }
    }

    pub fn get(&self, bit_pos: u8) -> Option<&NodeIndex> {
        if (self.bitmap & (1u64 << bit_pos)) != 0 {
            let mask = (1u64 << bit_pos) - 1;
            let array_idx = (self.bitmap & mask).count_ones() as usize;
            Some(&self.positions[array_idx])
        } else {
            None
        }
    }

    pub fn new() -> Self {
        TrieEntryG {
            bitmap: 0,
            positions: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Trie(Vec<TrieEntry>);

impl Trie {
    pub fn new() -> Self {
        let mut t = Trie(Vec::new());
        let v = vec![(
            0,
            NodeIndex {
                index: 0,
                dictionary_index: 0,
                terminated: false,
            },
        )]; //root node
        let tt = TrieEntryV(v);
        t.0.push(TrieEntry::TrieEntryV(tt));
        t
    }

    pub fn add_word(&mut self, word: &str) {
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
                        dictionary_index: 0,
                        terminated,
                    },
                )];
                let tt = TrieEntryV(v);
                self.0.push(TrieEntry::TrieEntryV(tt));
                let position = self.0.len() as u32 - 1;
                self.0[prev_row].update_index(prev_c, position);
                prev_c = c;
                prev_row = position as usize;
                continue;
            }
            prev_c = c;
            prev_row = curr_row;
            let entry = &mut self.0[curr_row];
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
                    dictionary_index: 0,
                    terminated,
                };
                entry.add(c, ni);
            }
        }
    }
}
