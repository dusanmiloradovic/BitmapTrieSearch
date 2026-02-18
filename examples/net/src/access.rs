use dictionary_bitmap_trie::dictionary::{AttributeSearch, Dictionary};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct OwnedSearchResult {
    pub term: String,
    pub attribute: String,
    pub original_entry: String,
    pub attribute_index: usize,
    pub position: usize,
}

pub struct DictionaryMap {
    map: HashMap<String, Arc<RwLock<Dictionary>>>,
}

impl DictionaryMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn create_dictionary(
        &mut self,
        dictionary_id: &str,
        attributes: Vec<(String, AttributeSearch)>,
    ) -> Result<(), String> {
        if self.map.contains_key(dictionary_id) {
            return Err(format!(
                "Dictionary with id '{}' already exists",
                dictionary_id
            ));
        }

        let dictionary = Dictionary::new(attributes);
        self.map
            .insert(dictionary_id.to_string(), Arc::new(RwLock::new(dictionary)));
        Ok(())
    }

    pub fn add_entry(
        &mut self,
        dictionary_id: &str,
        entry_data: HashMap<String, String>,
    ) -> Result<(), String> {
        match self.map.get(dictionary_id) {
            Some(dict_arc) => {
                let mut dict = dict_arc.write().unwrap();
                dict.add_dictionary_entry(entry_data);
                Ok(())
            }
            None => Err(format!("Dictionary with id '{}' not found", dictionary_id)),
        }
    }

    pub fn search(
        &self,
        dictionary_id: &str,
        term: &str,
    ) -> Result<Vec<OwnedSearchResult>, String> {
        match self.map.get(dictionary_id) {
            Some(dict_arc) => {
                let dict = dict_arc.read().unwrap();
                let results = dict.search(term);
                let owned_results = results
                    .into_iter()
                    .map(|sr| OwnedSearchResult {
                        term: sr.term.to_string(),
                        attribute: sr.attribute.to_string(),
                        original_entry: sr.original_entry.to_string(),
                        attribute_index: sr.attribute_index,
                        position: sr.position,
                    })
                    .collect();
                Ok(owned_results)
            }
            None => Err(format!("Dictionary with id '{}' not found", dictionary_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dictionary_bitmap_trie::dictionary::AttributeSearch;

    #[test]
    fn test_dictionary_map_basic_functionality() {
        let mut dict_map = DictionaryMap::new();

        // Test creating a dictionary
        let attributes = vec![
            ("name".to_string(), AttributeSearch::Multiple),
            ("surname".to_string(), AttributeSearch::Exact),
            ("id".to_string(), AttributeSearch::None),
        ];

        let result = dict_map.create_dictionary("test_dict", attributes);
        assert!(result.is_ok());

        // Test adding an entry
        let mut entry_data = HashMap::new();
        entry_data.insert("name".to_string(), "John Doe".to_string());
        entry_data.insert("surname".to_string(), "Doe".to_string());
        entry_data.insert("id".to_string(), "123".to_string());

        let result = dict_map.add_entry("test_dict", entry_data);
        assert!(result.is_ok());

        // Test searching
        let search_results = dict_map.search("test_dict", "John");
        assert!(search_results.is_ok());
        let results = search_results.unwrap();
        assert!(results.len() > 0);
        assert_eq!(results[0].term, "John Doe");
    }

    #[test]
    fn test_dictionary_not_found() {
        let dict_map = DictionaryMap::new();
        let result = dict_map.search("non_existent", "test");
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("not found"));
    }
}
