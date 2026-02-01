use std::collections::HashMap;

struct DictionaryEntry(Vec<String>);
// each attribute of a dictionaryentry is one string in a vector, the order is defined in the dictionary
// attribute is mapped to a usize, that is a position in the vector

pub struct Dictionary{
    entries: Vec<DictionaryEntry>,
    attribute_map: HashMap<String, usize>,
}