# BitmapTrie

A high-performance trie data structure library for efficient dictionary operations with bitmap-based encoding.

## Library Structure

This library provides core data structures for building fast dictionary and autocomplete systems:

- **`dictionary`** - High-level dictionary abstraction with configurable attribute search modes
- **`trie`** (internal) - Low-level bitmap trie implementation 
- **`encoding`** (internal) - Character encoding utilities

## Features

- **Fast Prefix Search**: Bitmap-based trie implementation for efficient prefix searching
- **Structured Data**: Support for dictionary entries with multiple named attributes
- **Flexible Search Modes**: Different search behaviors per attribute (exact, multiple word, none)
- **Memory Efficient**: Bitmap encoding reduces memory footprint
- **Thread Safe**: Dictionary can be safely shared across threads

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

See the `examples/net` directory for a complete networked dictionary service implementation demonstrating:

- TCP server/client architecture
- JSON-based communication protocol
- Multi-threaded dictionary operations
- Real-world usage patterns

To run the network example:

```bash
cd examples/net

# Terminal 1 - Start server
cargo run --bin server

# Terminal 2 - Run client
cargo run --bin client
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

## Dependencies

- `serde` - Serialization support
- `serde_json` - JSON serialization