# CSV Example for BitmapTrie

This example demonstrates how to use the `bitmap_trie` library with CSV data for fast searching across tabular data.

## Features

- **CSV Loading**: Parse CSV files and populate bitmap_trie dictionaries
- **Flexible Search Modes**: Configure different search behaviors per column
- **Fast Autocomplete**: Efficient prefix searching across CSV data
- **Type Safety**: Uses Rust's type system and the popular `csv` crate

## Dependencies

- [`csv`](https://crates.io/crates/csv) - The most popular CSV parsing library for Rust
- [`serde`](https://crates.io/crates/serde) - Serialization framework
- `bitmap_trie` - Our core library

## Usage

### Running the Example

```bash
cargo run
```

This will load sample employee data and demonstrate various search patterns.

### Running Tests

```bash
cargo test
```

## Example Code

```rust
use bitmap_trie::dictionary::AttributeSearch;
use csvexample::CsvDictionary;

// Configure search behavior per column
let attributes = vec![
    ("name".to_string(), AttributeSearch::Multiple),     // Search words in name
    ("company".to_string(), AttributeSearch::Exact),     // Prefix match company
    ("salary".to_string(), AttributeSearch::None),       // Metadata only
];

// Create and load dictionary
let mut dict = CsvDictionary::new(attributes);
let reader = std::fs::File::open("data.csv")?;
dict.load_from_csv(reader, true)?;

// Search across all indexed columns
let results = dict.search("John");
```

## Search Modes Explained

### `AttributeSearch::Multiple`
Perfect for text fields like names, descriptions, or addresses:
- Splits text into words
- Creates searchable n-grams 
- Example: "John Doe" becomes searchable by "John" and "Doe"

### `AttributeSearch::Exact` 
Ideal for categorical data like company names, cities, or status fields:
- Matches from the beginning of the field
- Example: "San Francisco" matches "San" but not "Francisco"

### `AttributeSearch::None`
For metadata that shouldn't be searchable:
- Stores the data but doesn't index it
- Good for IDs, timestamps, or computed values
- Reduces memory usage and search noise

## Use Cases

1. **Employee Directory**: Search by name, department, or location
2. **Product Catalog**: Find products by name, category, or description
3. **Customer Database**: Locate customers by company, contact info, or region
4. **Inventory Management**: Search parts by name, supplier, or specifications
5. **Log Analysis**: Find log entries by service, error type, or message content

## Performance Benefits

- **Memory Efficient**: Bitmap encoding reduces memory footprint
- **Fast Prefix Search**: O(k) lookup time where k is the prefix length
- **Concurrent Access**: Thread-safe operations for multi-user scenarios
- **Incremental Loading**: Add new CSV records without rebuilding the entire index

## Real-World Integration

```rust
// Load from file
let file = std::fs::File::open("employees.csv")?;
dict.load_from_csv(file, true)?;

// Load from HTTP response
let response = reqwest::get("https://api.example.com/data.csv").await?;
dict.load_from_csv(response, true)?;

// Load from string
let csv_data = "name,email\nJohn,john@example.com";
let cursor = std::io::Cursor::new(csv_data);
dict.load_from_csv(cursor, true)?;
```