use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddWordCommand {
    pub trie_id: String,
    pub word: String,
    pub dictionary: String,
    pub dictionary_index: u32,
    pub dictionary_attribute: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchCommand {
    pub trie_id: String,
    pub term: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddWordResponse {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<(String, String)>, //word, dictionary
    pub error: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    AddWord(AddWordCommand),
    Search(SearchCommand),
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    AddWord(AddWordResponse),
    Search(SearchResponse),
}

pub trait CommandHandler {
    fn handle(&mut self, command: Command) -> Response;
}
