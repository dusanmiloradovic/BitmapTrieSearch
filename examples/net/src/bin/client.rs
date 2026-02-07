use netexample::command::{
    AddDictionaryEntryCommand, Command, CreateDictionaryCommand, Response, SearchCommand,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Write};
use std::net::TcpStream;

fn create_dictionary_client(
    stream: &TcpStream,
    dictionary_id: &str,
    attributes: Vec<(String, String)>,
) {
    let reader = BufReader::new(stream);
    let writer = BufWriter::new(stream);
    let mut serializer = serde_json::Serializer::new(writer);
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let c = Command::CreateDictionary(CreateDictionaryCommand {
        dictionary_id: dictionary_id.to_string(),
        attributes,
    });
    c.serialize(&mut serializer).unwrap();
    serializer.into_inner().flush().unwrap();
    let response = Response::deserialize(&mut deserializer).unwrap();
    println!("Create dictionary response: {:?}", response);
}

fn add_dictionary_entry_client(
    stream: &TcpStream,
    dictionary_id: &str,
    entry_data: HashMap<String, String>,
) {
    let reader = BufReader::new(stream);
    let writer = BufWriter::new(stream);
    let mut serializer = serde_json::Serializer::new(writer);
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let c = Command::AddDictionaryEntry(AddDictionaryEntryCommand {
        dictionary_id: dictionary_id.to_string(),
        entry_data,
    });
    c.serialize(&mut serializer).unwrap();
    serializer.into_inner().flush().unwrap();
    let response = Response::deserialize(&mut deserializer).unwrap();
    println!("Add entry response: {:?}", response);
}

fn search_term_client(stream: &TcpStream, dictionary_id: &str, term: &str) {
    let reader = BufReader::new(stream);
    let writer = BufWriter::new(stream);
    let mut serializer = serde_json::Serializer::new(writer);
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let c = Command::Search(SearchCommand {
        dictionary_id: dictionary_id.to_string(),
        term: term.to_string(),
    });
    c.serialize(&mut serializer).unwrap();
    serializer.into_inner().flush().unwrap();
    let r = Response::deserialize(&mut deserializer).unwrap();
    println!("{:?}", r);
}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:4444").unwrap();

    // First create a dictionary with attributes
    create_dictionary_client(
        &stream,
        "names",
        vec![
            ("name".to_string(), "multiple".to_string()),
            ("surname".to_string(), "exact".to_string()),
            ("id".to_string(), "none".to_string()),
        ],
    );

    // Add some dictionary entries
    let mut entry1 = HashMap::new();
    entry1.insert("name".to_string(), "Dusan Milutinovic".to_string());
    entry1.insert("surname".to_string(), "Milutinovic".to_string());
    entry1.insert("id".to_string(), "1".to_string());
    add_dictionary_entry_client(&stream, "names", entry1);

    let mut entry2 = HashMap::new();
    entry2.insert("name".to_string(), "Dragan Miloradovic".to_string());
    entry2.insert("surname".to_string(), "Miloradovic".to_string());
    entry2.insert("id".to_string(), "2".to_string());
    add_dictionary_entry_client(&stream, "names", entry2);

    let mut entry3 = HashMap::new();
    entry3.insert("name".to_string(), "Petar Petrovic".to_string());
    entry3.insert("surname".to_string(), "Petrovic".to_string());
    entry3.insert("id".to_string(), "3".to_string());
    add_dictionary_entry_client(&stream, "names", entry3);

    // Search for terms
    search_term_client(&stream, "names", "Dusan");
    search_term_client(&stream, "names", "Mil");
    search_term_client(&stream, "names", "Petrovic");
}
