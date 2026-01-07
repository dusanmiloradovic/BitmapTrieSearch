use crate::encoding::{CHARS, idx};
use std::fmt;
#[derive(Clone, Debug, Copy)]
struct NodeIndex {
    index: u32,            //0 - leaf node
    dictionary_index: u32, // 0 for no index -> if terminated, this is a link to an actual dictionary entry
    terminated: bool,      // it can be terminated for one word, and still continue in the trie
}

const MAX_DIRECT_ENTRIES: usize = 5;
const MAX_SEARCH_RESULTS: usize = 10;

// TODO add types of dictionary entries
#[derive(Clone)]
struct TrieEntryG {
    bitmap: u64,
    positions: Vec<NodeIndex>,
}

struct TrieEntryV(Vec<(u8, NodeIndex)>);

#[derive(Debug)]
pub struct TrieSearchResult {
    word: String,
    dictionary_index: u32,
}

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

impl fmt::Debug for TrieEntryG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TrieEntryG: [")?;
        for i in 0..64 {
            let bit = self.bitmap & (1 << i);
            if bit != 0 {
                let c = CHARS.chars().nth(i as usize).unwrap();
                let z = self.get(i as u8).unwrap();
                write!(f, "({},{:?}),", c, z)?;
            }
        }
        write!(f, "]")
    }
}

trait TrieEntryOp {
    fn find(&self, c: char) -> Option<NodeIndex>;
    fn add(&mut self, c: char, ni: NodeIndex);
    fn update_index(&mut self, c: char, index: u32);
    fn update_terminated(&mut self, c: char, terminated: bool);
    fn get_all(&self) -> Vec<(char, NodeIndex)>;
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
            TrieEntry::TrieEntryG(g) => {
                g.insert_at(idx(c), ni);
            }
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
            TrieEntry::TrieEntryG(g) => {
                let pos = idx(c);
                match g.get(pos) {
                    Some(ni) => {
                        let mut nim = *ni;
                        nim.index = index;
                        g.insert_at(pos, nim);
                    }
                    None => {
                        let ni = NodeIndex {
                            index,
                            terminated: false,
                            dictionary_index: 0,
                        };
                        g.insert_at(pos, ni);
                    }
                }
            }
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
            TrieEntry::TrieEntryG(g) => {
                if let Some(ni) = g.get(ix) {
                    let mut nim = *ni;
                    nim.terminated = terminated;
                    g.insert_at(ix, nim);
                }
            }
        }
    }

    fn get_all(&self) -> Vec<(char, NodeIndex)> {
        let mut ret: Vec<(char, NodeIndex)> = Vec::new();
        match self {
            TrieEntry::TrieEntryV(v) => {
                for x in v.0.iter() {
                    let c = CHARS.chars().nth(x.0 as usize).unwrap();
                    ret.push((c, x.1));
                }
            }
            TrieEntry::TrieEntryG(g) => {
                for i in 0..64 {
                    let bit = g.bitmap & (1 << i);
                    if bit != 0 {
                        let c = CHARS.chars().nth(i as usize).unwrap();
                        let z = g.get(i as u8).unwrap();
                        ret.push((c, *z));
                    }
                }
            }
        }
        ret
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

    pub fn promote(trieEntry: &TrieEntryV) -> Self {
        let mut entry = TrieEntryG {
            bitmap: 0,
            positions: Vec::new(),
        };
        for r in &trieEntry.0 {
            let (c, node) = r;
            entry.insert_at(*c, *node);
        }
        entry
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
                if let TrieEntry::TrieEntryV(v) = entry {
                    if v.0.len() >= MAX_DIRECT_ENTRIES {
                        let promoted = TrieEntryG::promote(v);
                        self.0[curr_row] = TrieEntry::TrieEntryG(promoted);
                    }
                }
            }
        }
    }
    pub fn search(&self, term: &str) -> Vec<TrieSearchResult> {
        let mut res = Vec::new();
        let mut curr_row = 0;
        for c in term.chars() {
            let entry = self.0[curr_row].find(c);
            match entry {
                None => return res,
                Some(ni) => {
                    if ni.terminated {
                        res.push(TrieSearchResult {
                            word: term.to_string(),
                            dictionary_index: ni.dictionary_index,
                        });
                    }
                    curr_row = ni.index as usize;
                }
            }
            // if any word was found it will be in the return vector, from here return all the children (filtered with terminated)
        }
        let entry = &self.0[curr_row];
        let children = entry.get_all();
        let mut bfs_stack:Vec<(String,NodeIndex)> = Vec::new();
        for (c, ni) in children {
            let w = term.to_string() + &c.to_string();
            bfs_stack.push((w,ni));
        }
        while bfs_stack.len() > 0 && res.len() < MAX_SEARCH_RESULTS {
           let e =  bfs_stack.pop();
            match e {
               None => break,
               Some((w,ni)) => {
                   if ni.terminated {
                       res.push(TrieSearchResult {
                           word: w.clone(),
                           dictionary_index: ni.dictionary_index,
                       });
                   }
                   if ni.index != 0 {
                           let entry = &self.0[ni.index as usize];
                           let children = entry.get_all();
                           for (c, ni) in children {
                               bfs_stack.push((w.to_string() + &c.to_string(),ni));
                           }
                       }
                   }
               }

        }
        res
    }
}
