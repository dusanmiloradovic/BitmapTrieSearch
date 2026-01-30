use crate::encoding::{CHARS, idx};
use std::fmt;

#[derive(Clone, Debug, Copy)]
pub struct NodeIndex {
    pub index: u32, //0 - leaf node
    pub terminated: bool, // it can be terminated for one word, and still continue in the trie
                // TODO no point keeping both terminated and trie_map_index, since every terminated will point to
                // dictionary map
}

pub const MAX_DIRECT_ENTRIES: usize = 5;

#[derive(Clone)]
pub struct TrieEntryG {
    pub bitmap: u64,
    pub positions: Vec<NodeIndex>,
}

pub struct TrieEntryV(pub Vec<(u8, NodeIndex)>);

impl fmt::Debug for TrieEntryV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Here we can choose to hide the actual data
        let r = &self.0;
        write!(f, "TrieEntryV: [")?;
        for (_, z) in r.iter().enumerate() {
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

pub trait TrieEntryOp {
    fn find(&self, c: char) -> Option<NodeIndex>;
    fn add(&mut self, c: char, ni: NodeIndex);
    fn update_index(&mut self, c: char, index: u32);
    fn update_terminated(&mut self, c: char, terminated: bool);
    fn get_all(&self) -> Vec<(char, NodeIndex)>;
    fn remove(&mut self, c: char) -> bool; //if true is returned, the trie entry is empty, can be removed (mark row as free)
}

#[derive(Debug)]
pub enum TrieEntry {
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
        let pos = idx(c);
        match self {
            TrieEntry::TrieEntryV(v) => {
                for vv in v.0.iter_mut() {
                    if vv.0 == pos {
                        let mut ni = vv.1;
                        ni.index = index;
                        vv.1 = ni;
                    }
                }
            }
            TrieEntry::TrieEntryG(g) => match g.get(pos) {
                Some(ni) => {
                    let mut nim = *ni;
                    nim.index = index;
                    g.insert_at(pos, nim);
                }
                None => {
                    let ni = NodeIndex {
                        index,
                        terminated: false,
                    };
                    g.insert_at(pos, ni);
                }
            },
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

    fn remove(&mut self, c: char) -> bool {
        let pos = idx(c);
        match self {
            TrieEntry::TrieEntryV(v) => {
                v.0.retain(|x| x.0 != pos);
                v.0.len() == 0
            }
            TrieEntry::TrieEntryG(g) => g.remove_at(pos),
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

    pub fn remove_at(&mut self, bit_pos: u8) -> bool {
        if (self.bitmap & (1u64 << bit_pos)) == 0 {
            // Bit not set, nothing to remove
            return self.bitmap == 0;
        }
        let mask = (1u64 << bit_pos) - 1;
        // The index in the Vec is the number of set bits to the right of bit_pos
        let array_idx = (self.bitmap & mask).count_ones() as usize;
        self.positions.remove(array_idx);
        let mut mask = u64::MAX;
        mask &= !(1u64 << bit_pos);

        self.bitmap &= mask;
        self.bitmap == 0
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

    pub fn promote(trie_entry: &TrieEntryV) -> Self {
        let mut entry = TrieEntryG {
            bitmap: 0,
            positions: Vec::new(),
        };
        for r in &trie_entry.0 {
            let (c, node) = r;
            entry.insert_at(*c, *node);
        }
        entry
    }
}
