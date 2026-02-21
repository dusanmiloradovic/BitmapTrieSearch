/// Configuration for search behavior. All fields have sensible defaults.
#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub max_search_results: usize,
    pub max_direct_entries: usize,
    pub min_term_length: usize,
    pub default_multiple_search_length: usize,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            max_search_results: 10,
            max_direct_entries: 5, // how long before we promote to a TrieEntryG
            min_term_length: 3,
            default_multiple_search_length: 3,
        }
    }
}