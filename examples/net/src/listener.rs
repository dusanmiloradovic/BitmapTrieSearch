use crate::access::DictionaryMap;
use crate::command::Command::{AddDictionaryEntry, CreateDictionary, Search};
use crate::command::{
    AddDictionaryEntryResponse, Command, CreateDictionaryResponse, Response, SearchResponse,
    SearchResultItem,
};
use bitmap_trie::dictionary::AttributeSearch;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

pub fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4444")?;

    let dict_map = crate::access::DictionaryMap::new();
    let arc_dict_map = Arc::new(RwLock::new(dict_map));

    // accept connections and process them serially
    for res_stream in listener.incoming() {
        let arc_clone = Arc::clone(&arc_dict_map);
        thread::spawn(move || match res_stream {
            Ok(stream) => handle_client(stream, &arc_clone),
            Err(e) => panic!("{:?}", e),
        });
    }
    Ok(())
}

fn handle_client(stream: TcpStream, cloned_arc: &Arc<RwLock<DictionaryMap>>) {
    let reader = BufReader::new(&stream);
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    // This allows handling multiple commands sent over the same connection
    while let Ok(command) = Command::deserialize(&mut deserializer) {
        let response = match command {
            CreateDictionary(create_dict) => {
                let mut dict_map = cloned_arc.write().unwrap();
                let attributes: Vec<(String, AttributeSearch)> = create_dict
                    .attributes
                    .into_iter()
                    .map(|(name, search_type)| {
                        let attr_search = match search_type.to_lowercase().as_str() {
                            "exact" => AttributeSearch::Exact,
                            "multiple" => AttributeSearch::Multiple,
                            _ => AttributeSearch::None,
                        };
                        (name, attr_search)
                    })
                    .collect();

                match dict_map.create_dictionary(&create_dict.dictionary_id, attributes) {
                    Ok(_) => Response::CreateDictionary(CreateDictionaryResponse {
                        success: true,
                        error: None,
                    }),
                    Err(e) => Response::CreateDictionary(CreateDictionaryResponse {
                        success: false,
                        error: Some(e),
                    }),
                }
            }
            AddDictionaryEntry(add_entry) => {
                let mut dict_map = cloned_arc.write().unwrap();
                match dict_map.add_entry(&add_entry.dictionary_id, add_entry.entry_data) {
                    Ok(_) => Response::AddDictionaryEntry(AddDictionaryEntryResponse {
                        success: true,
                        error: None,
                    }),
                    Err(e) => Response::AddDictionaryEntry(AddDictionaryEntryResponse {
                        success: false,
                        error: Some(e),
                    }),
                }
            }
            Search(search_command) => {
                let dict_map = cloned_arc.read().unwrap();
                match dict_map.search(&search_command.dictionary_id, &search_command.term) {
                    Ok(results) => {
                        let search_results: Vec<SearchResultItem> = results
                            .into_iter()
                            .map(|sr| SearchResultItem {
                                term: sr.term,
                                attribute: sr.attribute,
                                original_entry: sr.original_entry,
                                dictionary_id: search_command.dictionary_id.clone(),
                            })
                            .collect();
                        Response::Search(SearchResponse {
                            results: search_results,
                            error: None,
                        })
                    }
                    Err(e) => Response::Search(SearchResponse {
                        results: vec![],
                        error: Some(e),
                    }),
                }
            }
        };
        let mut serializer = serde_json::Serializer::new(&stream);
        response.serialize(&mut serializer).unwrap();
        serializer.into_inner().flush().unwrap();
    }
}
