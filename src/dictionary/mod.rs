use crate::trie::Trie;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

const DEFAULT_MULTIPLE_SEARCH_LENGTH: usize = 3;

struct DictionaryEntry(HashMap<usize, String>);
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
        if j + DEFAULT_MULTIPLE_SEARCH_LENGTH < z.len() {
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
    pub fn add_dictionary_entry(&mut self, data: HashMap<String, String>) {
        let mut m: HashMap<usize, String> = HashMap::new();
        data.keys().for_each(|k| {
            if let Some((u, attr_s)) = self.attribute_map.get(k) {
                m.insert(*u, data[k].clone());
                match attr_s {
                    AttributeSearch::None => (),
                    AttributeSearch::Exact => {
                        let mut l = self.trie.write().unwrap();
                        l.add_word(&data[k], m.len() as u32 + 1, *u as u8, 0);
                    }
                    AttributeSearch::Multiple => {
                        let v = split_word(&data[k]);
                        for (s, pos) in v {
                            let mut l = self.trie.write().unwrap();
                            l.add_word(&s, m.len() as u32 + 1, *u as u8, pos as u16);
                        }
                    }
                }
            }
        });
        if m.len() == 0 {
            return;
        }
        self.entries.push(DictionaryEntry(m));
    }
}

#[cfg(test)]
mod test {
    use crate::dictionary::{AttributeSearch, Dictionary, split_word};
    use std::collections::HashMap;

    fn prepare_dictionary() -> Dictionary {
        let m = vec![
            ("car".to_string(), AttributeSearch::Multiple),
            ("manufacturer".to_string(), AttributeSearch::Exact),
            ("serial_number".to_string(), AttributeSearch::None),
        ];
        let mut d = Dictionary::new(m);
        d.add_dictionary_entry(HashMap::from([
            ("manufacturer".to_string(), "Toyota".to_string()),
            ("car".to_string(), "Corolla".to_string()),
            ("serial_number".to_string(), "123456".to_string()),
        ]));
        d.add_dictionary_entry(HashMap::from([
            ("manufacturer".to_string(), "Subaru".to_string()),
            ("car".to_string(), "Outback".to_string()),
            ("serial_number".to_string(), "1234567".to_string()),
        ]));
        d.add_dictionary_entry(HashMap::from([
            ("manufacturer".to_string(), "Honda".to_string()),
            ("car".to_string(), "Accord".to_string()),
            ("serial_number".to_string(), "123458".to_string()),
        ]));
        d
    }
    #[test]
    fn test_split_word() {
        let w = "ab bc cd ef gh kl";
        let g = split_word(w);
        let expected: Vec<(&str, usize)> = vec![
            ("ab bc cd", 0),
            ("bc cd ef", 3),
            ("cd ef gh", 6),
            ("ef gh kl", 9),
            ("gh kl", 12),
            ("kl", 15),
        ];
        assert_eq!(
            g.iter()
                .map(|(s, pos)| (s.as_str(), *pos))
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn test_dictionary_addition() {
        let d = prepare_dictionary();
        let lock = d.trie.read().unwrap();
        let rez = lock
            .search("CO")
            .iter()
            .map(|x| x.word.clone())
            .collect::<Vec<String>>();
        assert_eq!(rez, vec!["COROLLA".to_string()]);
        let rez2 = lock
            .search("123")
            .iter()
            .map(|x| x.word.clone())
            .collect::<Vec<String>>();
        assert_eq!(rez2.len(), 0);
    }
}
