use crate::access::TrieMap;
use crate::command::{AddWordResponse, Command, Response, SearchResponse};
use crate::command::Command::{AddWord, Search};
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

pub fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4444")?;

    let trieMap = crate::access::TrieMap::new();
    let arcTrieMap = Arc::new(RwLock::new(trieMap));

    // accept connections and process them serially
    for res_stream in listener.incoming() {
        let arc_clone = Arc::clone(&arcTrieMap);
        thread::spawn(move || {
            match res_stream {
                Ok(stream) => handle_client(stream, &arc_clone),
                Err(e) => panic!("{:?}", e),
            }
        });
    }
    Ok(())
}

fn handle_client(stream: TcpStream, cloned_arc: & Arc<RwLock<TrieMap>>) {
        let reader = BufReader::new(&stream);
        let mut deserializer = serde_json::Deserializer::from_reader(reader);
        let mut serializer = serde_json::Serializer::new(&stream);

        // This allows handling multiple commands sent over the same connection
        while let Ok(command) = Command::deserialize(&mut deserializer) {
            println!("Received: {:?}", command);
            let response = match command {
                AddWord(ad_w)=> {
                    let mut w = cloned_arc.write().unwrap();
                    w.add_word(&ad_w.trieId,&ad_w.word);
                    Response::AddWord(AddWordResponse { success: true, error: None })
                },
                Search(search_command)=>{
                    let r = cloned_arc.read().unwrap();
                    let res= r.search(&search_command.trieId,&search_command.term);
                    match res{
                        None=>Response::Search(SearchResponse{results: vec![], error: Some("No results".to_string())}),
                        Some(r)=>{
                            let b:Vec<(String,String)>=  r.iter().map(|x| (x.word.clone(),"".to_string())).collect();
                            Response::Search(SearchResponse{results: b, error: None})
                        }
                    }
                },
            };
            response.serialize(&mut serializer).unwrap();
        }
    }
