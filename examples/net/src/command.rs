use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddDictionaryEntryCommand {
    pub dictionary_id: String,
    pub entry_data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchCommand {
    pub dictionary_id: String,
    pub term: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDictionaryCommand {
    pub dictionary_id: String,
    pub attributes: Vec<(String, String)>, // (attribute_name, search_type)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddDictionaryEntryResponse {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResultItem>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResultItem {
    pub term: String,
    pub attribute: String,
    pub original_entry: String,
    pub dictionary_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDictionaryResponse {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    CreateDictionary(CreateDictionaryCommand),
    AddDictionaryEntry(AddDictionaryEntryCommand),
    Search(SearchCommand),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    CreateDictionary(CreateDictionaryResponse),
    AddDictionaryEntry(AddDictionaryEntryResponse),
    Search(SearchResponse),
}

pub trait CommandHandler {
    fn handle(&mut self, command: Command) -> Response;
}
