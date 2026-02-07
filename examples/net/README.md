# Network Example for BitmapTrie

This is an example project demonstrating how to use the `bitmap_trie` library to create a networked dictionary service.

## Structure

- `src/access.rs` - `DictionaryMap` for managing multiple dictionaries
- `src/command.rs` - Command and response structures for the network protocol
- `src/listener.rs` - TCP server implementation that handles dictionary commands
- `src/bin/server.rs` - Server binary
- `src/bin/client.rs` - Client binary with example usage

## Features

This example demonstrates:
- Creating dictionaries with different attribute types (exact, multiple, none)
- Adding structured entries to dictionaries
- Searching across dictionary entries
- Network protocol using JSON over TCP
- Multi-threaded server handling multiple clients

## Usage

### Running the Server

```bash
cargo run --bin server
```

The server will listen on `127.0.0.1:4444`.

### Running the Client

```bash
cargo run --bin client
```

The client will connect to the server and demonstrate:
1. Creating a dictionary with attributes
2. Adding entries with structured data
3. Performing searches

### Example Workflow

1. **Create Dictionary**: Define attributes and their search types
   ```json
   {
     "CreateDictionary": {
       "dictionary_id": "names",
       "attributes": [
         ["name", "multiple"],
         ["surname", "exact"],
         ["id", "none"]
       ]
     }
   }
   ```

2. **Add Entries**: Insert structured data
   ```json
   {
     "AddDictionaryEntry": {
       "dictionary_id": "names",
       "entry_data": {
         "name": "John Doe",
         "surname": "Doe", 
         "id": "123"
       }
     }
   }
   ```

3. **Search**: Find entries by partial matches
   ```json
   {
     "Search": {
       "dictionary_id": "names",
       "term": "John"
     }
   }
   ```

## Attribute Types

- **exact**: Matches from the beginning of the attribute value
- **multiple**: Splits attribute into words and searches consecutive word combinations
- **none**: Attribute is stored but not searchable (metadata only)

## Testing

```bash
cargo test
```