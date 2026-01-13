use crate::command::Command;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use serde::Deserialize;

pub fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4444")?;

    let trieMap = crate::access::TrieMap::new();
    let arcTrieMap = std::sync::Arc::new(trieMap);

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

fn handle_client(stream: TcpStream, cloned_arc: &std::sync::Arc<crate::access::TrieMap>) {
        let reader = BufReader::new(&stream);
        let mut deserializer = serde_json::Deserializer::from_reader(reader);

        // This allows handling multiple commands sent over the same connection
        while let Ok(command) = Command::deserialize(&mut deserializer) {
            println!("Received: {:?}", command);
            // ... process command ...
            write!(&stream, "Success\n").expect("Failed to write");
        }
    }
