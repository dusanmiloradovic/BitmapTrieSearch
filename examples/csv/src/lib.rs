use bitmap_trie::dictionary::{AttributeSearch, Dictionary};
use csv::ReaderBuilder;

use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

/// A simple CSV-based dictionary loader that can read CSV files
/// and populate bitmap_trie dictionaries for fast searching
pub struct CsvDictionary {
    dictionary: Dictionary,
}

impl CsvDictionary {
    /// Create a new CSV dictionary with specified attribute configurations
    pub fn new(attributes: Vec<(String, AttributeSearch)>) -> Self {
        let dictionary = Dictionary::new(attributes);
        Self { dictionary }
    }

    /// Load data from CSV reader into the dictionary
    pub fn load_from_csv<R: Read>(
        &self,
        reader: R,
        has_headers: bool,
    ) -> Result<usize, Box<dyn Error>> {
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(reader);

        let headers = if has_headers {
            csv_reader.headers()?.clone()
        } else {
            // If no headers, create generic column names
            let first_record = csv_reader.headers()?.clone();
            csv::StringRecord::from(
                (0..first_record.len())
                    .map(|i| format!("column_{}", i))
                    .collect::<Vec<_>>(),
            )
        };

        let mut count = 0;
        for result in csv_reader.records() {
            let record = result?;
            let mut entry_data = HashMap::new();

            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    entry_data.insert(header.to_string(), field.to_string());
                }
            }

            self.dictionary.add_dictionary_entry(entry_data);
            count += 1;
        }

        Ok(count)
    }

    /// Search the dictionary
    pub fn search(&self, term: &str) -> Vec<bitmap_trie::dictionary::SearchResult> {
        self.dictionary.search(term)
    }

    pub fn get(&self, id: &str) -> HashMap<String, String> {
        if let Ok(id) = id.parse::<usize>() {
            self.dictionary.get(id)
        } else {
            HashMap::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_csv_loading() {
        let csv_data = "name,city,country\nJohn Doe,New York,USA\nJane Smith,London,UK\nBob Johnson,Paris,France";

        let attributes = vec![
            ("name".to_string(), AttributeSearch::Multiple),
            ("city".to_string(), AttributeSearch::Exact),
            ("country".to_string(), AttributeSearch::Exact),
        ];

        let mut dict = CsvDictionary::new(attributes);
        let reader = Cursor::new(csv_data);

        let count = dict.load_from_csv(reader, true).unwrap();
        assert_eq!(count, 3);

        // Test searching
        let results = dict.search("John");
        assert!(results.len() > 0);

        let results = dict.search("New");
        assert!(results.len() > 0);
    }
}
