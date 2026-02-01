use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::trie::Trie;

const DEFAULT_MULTIPLE_SEARCH_LENGTH: usize = 3;

struct DictionaryEntry(Vec<String>);
// each attribute of a dictionaryentry is one string in a vector, the order is defined in the dictionary
// attribute is mapped to a usize, that is a position in the vector

pub enum AttributeSearch {
    None,     // don't include the attribute in search, but include it in result
    Exact,    // autocomplete has to be exact match from the beginning of attribute
    Multiple, // split the attribute into words and search
}
// Default multiple behavior:
// a b c d e -> (a,b,c), (b,c,d), (c,d,e), (d,e), e
// in other words the search is for the 3 consecutive words(we can define the different default)

pub struct Dictionary {
    entries: Vec<DictionaryEntry>,
    attribute_map: HashMap<String, (usize, AttributeSearch)>,
    trie: Arc<RwLock<Trie>>,
}

fn split_word(word: &str) -> Vec<(String, usize)> {
    let mut ret = Vec::new();
    let z = word.split_whitespace().collect::<Vec<&str>>();
    let mut position = 0;
    for j in 0..z.len() {
        // Find the position of this word part in the original string
        if let Some(pos) = word[position..].find(z[j]) {
            position += pos;
        }
        if j < z.len() - DEFAULT_MULTIPLE_SEARCH_LENGTH {
            ret.push((z[j..j + DEFAULT_MULTIPLE_SEARCH_LENGTH].join(" "), position));
        } else {
            ret.push((z[j..].join(" "), position));
        }
        
        // Move position past the current word
        position += z[j].len();
    }
    ret
}

impl Dictionary {
    pub fn new(attrs: Vec<(String, AttributeSearch)>) -> Dictionary {
        let mut attribute_map = HashMap::new();
        for (attr, search) in attrs {
            attribute_map.insert(attr, (attribute_map.len(), search));
        }
        Dictionary {
            entries: Vec::new(),
            attribute_map,
            trie: Arc::new(RwLock::new(Trie::new())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::dictionary::split_word;

    #[test]
    fn test_split_word() {
        let w = "ab bc cd ef gh kl";
        let g = split_word(w);
        let expected: Vec<(&str, usize)> = vec![("ab bc cd", 0), ("bc cd ef", 3), ("cd ef gh", 6), ("ef gh kl", 9), ("gh kl", 12),("kl",15)];
        assert_eq!(g.iter().map(|(s, pos)| (s.as_str(), *pos)).collect::<Vec<_>>(), expected);
    }
}
