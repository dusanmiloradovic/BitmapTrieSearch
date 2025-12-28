use crate::encoding::idx;
#[derive(Clone)]
struct NodeIndex {
    index: u32,            //0 termination
    dictionary_index: u32, // 0 for no index -> if terminated, this is a link to an actual dictionary entry
}

const MAX_DIRECT_ENTRIES: usize = 5;

// TODO add types of dictionary entries
#[derive(Clone)]
struct TrieEntryG {
    bitmap: u64,
    positions: Vec<NodeIndex>,
}

struct TrieEntryV(Vec<(u8, Option<NodeIndex>)>);

enum TrieEntry {
    TrieEntryG(TrieEntryG),
    TrieEntryV(TrieEntryV),
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

pub struct Trie(Vec<TrieEntry>);

impl Trie {
    pub fn new() -> Self {
        Trie(Vec::new())
    }

    pub fn add_word(&mut self, word: &str) {
        let mut nextRow: usize = 0;
        let mut currRow: usize = 0;
        let mut currCol: usize = 0;
        let mut prevRow: usize = 0;
        let mut prevCol: usize = 0;
        let mut should_update_prev_row = false;
        let mut should_update_curr_row = false;
        for (i, c) in word.chars().enumerate() {
            let char_idx = idx(c);
            prevRow = currRow;
            prevCol = currCol;
            should_update_prev_row = should_update_curr_row;
            should_update_curr_row = false;
            if nextRow == 0 {
                let v = vec![(char_idx, None)];
                let t = TrieEntryV(v);
                self.0.push(TrieEntry::TrieEntryV(t));
                should_update_curr_row = true;
                currRow = self.0.len() - 1;
                currCol = 0;
            } else {
                currRow = nextRow;
                nextRow = 0;
                let entry = &mut self.0[currRow];
                match entry {
                    TrieEntry::TrieEntryV(v) => {
                        let mut found = false;
                        for (j, z) in v.0.iter().enumerate() {
                            if z.0 == char_idx {
                                found = true;
                                if let Some(node) = &z.1 {
                                    nextRow = node.index as usize;
                                    currCol = j;
                                }
                            }
                        }
                        if !found {
                            v.0.push((char_idx, None));
                            should_update_curr_row = true;
                            currCol = v.0.len() - 1;
                            // TODO after currCol = 5 , migrate to TrieEntryG
                        }
                    }
                    _ => {}
                }
            }
            // For first char, it has to be in the first row
            if currRow == 0 || !should_update_prev_row {
                continue;
            }
            let previous_entry = &mut self.0[prevRow];
            match previous_entry {
                TrieEntry::TrieEntryV(v) => {
                    let previous_node = &mut v.0[prevCol].1;
                    if let Some(node) = previous_node {
                        node.index = currRow as u32;
                    }
                }
                _ => {}
            }
        }
    }
}
