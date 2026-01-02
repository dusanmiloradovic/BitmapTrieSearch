use crate::encoding::{CHARS, idx};
use std::fmt;
#[derive(Clone, Debug, Copy)]
struct NodeIndex {
    index: u32,            //0 termination
    dictionary_index: u32, // 0 for no index -> if terminated, this is a link to an actual dictionary entry
}

const MAX_DIRECT_ENTRIES: usize = 5;

// TODO add types of dictionary entries
#[derive(Clone, Debug)]
struct TrieEntryG {
    bitmap: u64,
    positions: Vec<NodeIndex>,
}

struct TrieEntryV(Vec<(u8, Option<NodeIndex>)>);


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

trait TrieEntryOp{
    fn find(&self, c:char) -> Option<NodeIndex>;
    fn add(&mut self, c:char, ni:Option<NodeIndex>);
    fn update(&mut self, c:char, ni:Option<NodeIndex>);
}

#[derive(Debug)]
enum TrieEntry {
    TrieEntryG(TrieEntryG),
    TrieEntryV(TrieEntryV),
}

impl TrieEntryOp for TrieEntry {
    fn find(&self, c:char) -> Option<NodeIndex> {
        let char_idx = idx(c);
        match self {
            TrieEntry::TrieEntryV(v) => {
                let m = &v.0;
                for vv in m.into_iter() {
                    if vv.0 == char_idx {
                        return vv.1;
                    }
                }
                return None;
            },
            TrieEntry::TrieEntryG(v) => {None} // TODO
        }
    }

    fn add(&mut self, c: char, ni: Option<NodeIndex>) {
        match self {
            TrieEntry::TrieEntryV(v) => {v.0.push((idx(c), ni))}
            _ => {} // TODO
        }
    }

    fn update(&mut self, c: char, ni: Option<NodeIndex>) {
        let ix = idx(c);
        match self{
            TrieEntry::TrieEntryV(v) => {
                for vv in v.0.iter_mut() {
                    if vv.0 == ix {
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
        let v = vec![(0, None)]; //root node
        let tt = TrieEntryV(v);
        t.0.push(TrieEntry::TrieEntryV(tt));
        t
    }

    pub fn add_word(&mut self, word: &str) {
        let mut curr_row=0;
        let mut next_row = 0;
        let mut prev_row= 0;
        let mut update_prev = false;
        let mut should_add =false;
        let mut prev_c:char = 0 as char;
        // once we add the new entry, all the leaf nodes will have to be created anew
        for (i,c) in word.chars().enumerate() {
            if should_add{
                let v = vec![(idx(c), None)];
                let tt = TrieEntryV(v);
                self.0.push(TrieEntry::TrieEntryV(tt));
                let position = self.0.len() as u32 - 1;
                let ni = Some(NodeIndex{index:position,dictionary_index:0});
                self.0[prev_row].update(prev_c, ni);
                print!("For {}, updating prev {}\n", c, prev_c);
                prev_c =c;
                continue;
            }
            prev_c =c;
            update_prev = false;
            prev_row = curr_row;
            curr_row = next_row;
            let entry = &mut self.0[curr_row];
            let existing = entry.find(c);
            if let Some(node) = existing {
                next_row = node.index as usize;
                continue;
            } else {
                should_add = true;
                entry.add(c, None);
            }
        }
    }
    
}
