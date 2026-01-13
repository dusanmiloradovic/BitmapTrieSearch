use serde::{Serialize, Deserialize};
use crate::access::TrieMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddWordCommand {
    pub trieId: String,
    pub word: String,
    pub dictionary: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchCommand {
    pub trieId: String,
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

impl CommandHandler for TrieMap {
    fn handle(&mut self, command: Command) -> Response {
        match command {
            Command::AddWord(w) => {
                self.add_word(&w.trieId, &w.word);
                Response::AddWord(AddWordResponse { success: true, error: None })
            }
            Command::Search(s)=>{
                match self.search(&s.trieId, &s.term){
                    None => Response::Search(SearchResponse{results: vec![], error: Some("No results".to_string())}),
                    Some(r)=>{
                       let b:Vec<(String,String)>=  r.iter().map(|x| (x.word.clone(),"".to_string())).collect();
                        Response::Search(SearchResponse{results: b, error: None})
                    }
                }
            }
        }
    }
}