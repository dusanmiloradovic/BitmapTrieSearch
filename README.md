# Dictionary BitmapTrie

This library for autocomplete and prefix search, backed by Bitmap Trie data structure.
The search data is stored in "Dictionary", that defines how the data is structured.


## Library Structure

This library provides core data structures for building fast dictionary and autocomplete systems:

- **`dictionary`** - Dictionary abstraction for defining search attributes and data structure
- **`trie`** (internal) - Low-level bitmap trie implementation 
- **`encoding`** (internal) - Character encoding utilities
### Dictionary
Before using the dictionary, we have to define which attributes the data has, and how it is going to be searched
- AttributeSearch::None means the data is not searchable, but its stored in Trie (like internal id for example)
- AttributeSearch::Exact - attribute is searchable, but the prefix has to exactly match (for example if we have a entry "john doe", "john" will match, but "doe" will not)
- AttributeSeach::Multiple - the text is split into words internally and searchable. Internally, the attribute is split into tuples of length 3 and stored in trie. This can be configured by changing DEFAULT_MULTIPLE_SEARCH_LENGTH. Search terms longer than this are filtered directly through dictionary
### Encoding
Bitmap trie data structure has a 64 bit mapping entry where each bit corresponds to a character. By default, all text is encoded to a ASCII subset of characters, and each grapheme cluster is mapped to one character. The default implementation supports only Latin scripts. To configure this, implement the Encoding trait
## Usage

```rust
use bitmap_trie::dictionary::{Dictionary, AttributeSearch};
use std::collections::HashMap;

// Create a dictionary with different attribute types
let attributes = vec![
    ("name".to_string(), AttributeSearch::Multiple),   // Split into words
    ("surname".to_string(), AttributeSearch::Exact),   // Exact prefix match
    ("id".to_string(), AttributeSearch::None),         // Metadata only
];

let mut dict = Dictionary::new(attributes);

// Add entries with structured data
let mut entry = HashMap::new();
entry.insert("name".to_string(), "John Doe".to_string());
entry.insert("surname".to_string(), "Doe".to_string());
entry.insert("id".to_string(), "123".to_string());

dict.add_dictionary_entry(entry);

// Search for entries
let results = dict.search("John");
for result in results {
    println!("Found: {} in attribute: {}", result.term, result.attribute);
}
```

## Attribute Search Types

- **`AttributeSearch::Exact`**: Matches from the beginning of the attribute value
- **`AttributeSearch::Multiple`**: Splits attribute into words and creates searchable n-grams
- **`AttributeSearch::None`**: Stores but doesn't index the attribute (metadata only)

## Examples

See the `examples/csv` directory for a book search example application

```bash
cd examples/csv

# Terminal 1 - Start server
cargo run

# Terminal 2 - Run client
cd examples/csv/frontend
npm run dev
```

## Building

```bash
# Build library
cargo build

# Run tests
cargo test

# Build examples
cd examples/net && cargo build
```