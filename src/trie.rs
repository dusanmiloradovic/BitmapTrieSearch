use crate::encoding::idx;
#[derive(Clone)]
struct NodeIndex {
    index: u32,            //0 termination
    dictionary_index: u32, // 0 for no index -> if terminated, this is a link to an actual dictionary entry
}

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
        let mut nextRow: usize = 0; //
        let mut prevRow: usize = 0;
        let mut prevCol: usize = 0;
        for (i, c) in word.chars().enumerate() {
            let char_idx = idx(c);
            // For first char, it has to be in the first row
            if self.0.is_empty() {
                let v = vec![(char_idx, None)];
                let t = TrieEntryV(v);
                self.0.push(TrieEntry::TrieEntryV(t));
                prevRow = nextRow;
                nextRow = 0;
            } else {
                if nextRow == 0 {
                    let v = vec![(char_idx, None)];
                    let t = TrieEntryV(v);
                    self.0.push(TrieEntry::TrieEntryV(t));
                    nextRow = self.0.len() - 1;
                    if i == 0 {
                        continue;
                    }
                    let prev_entry = &mut self.0[prevRow];
                    match prev_entry {
                        TrieEntry::TrieEntryV(v) => {
                            v.0[prevCol].1 = Some(NodeIndex { index: nextRow as u32, dictionary_index: 0 });
                        }
                        _ => {}
                    }
                }

                let entry = &mut self.0[0];
                match entry {
                    TrieEntry::TrieEntryV(v) => {
                        let mut found = false;
                        for (j, z) in v.0.iter().enumerate() {
                            if z.0 == char_idx {
                                found = true;
                                if let Some(node) = &z.1 {
                                    nextRow = node.index as usize;
                                    prevCol = j;
                                }
                                continue;
                            }
                        }
                        if !found {
                            v.0.push((char_idx, None));
                            prevCol = v.0.len() - 1;
                            prevRow = nextRow;
                            nextRow = 0; //
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
