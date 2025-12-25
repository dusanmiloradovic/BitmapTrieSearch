use crate::encoding::idx;
#[derive(Clone)]
struct NodeIndex {
    index: u32,                 //0 termination
    restOfWord: Option<String>, // only for terminated
    dictionaryIndex: u32, // 0 for no index -> if terminated, this is a link to an actual dictionary entry
}

// TODO add types of dictionary entries
#[derive(Clone)]
struct TrieEntry {
    bitmap: u64,
    positions: Vec<NodeIndex>,
}

impl TrieEntry {
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
        TrieEntry {
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
        for (i, c) in word.chars().enumerate() {
            let char_idx = idx(c);
            if i == 0 {
                if self.0.is_empty() {
                    self.0.push(TrieEntry::new());
                }
                let entry = &mut self.0[0];

            } else {
                print!("Nothing")
            }
        }
    }
}
