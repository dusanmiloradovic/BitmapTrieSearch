struct NodeIndex{
    index: u32, //0 termination
    restOfWord: Option<String>,// only for terminated
    dictionaryIndex: u32, // 0 for no index -> if terminated, this is a link to an actual dictionary entry
}

// TODO add types of dictionary entries
struct Trie {
    bitmap: u64,
    positions: Vec<NodeIndex>
}

