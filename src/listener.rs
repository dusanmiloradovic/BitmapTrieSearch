use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

pub fn listen () ->  std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:4444")?;

    let trieMap = crate::access::TrieMap::new();
    let arcTrieMap = std::sync::Arc::new(trieMap);

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?, &arcTrieMap);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, arcTrieMap: &std::sync::Arc<crate::access::TrieMap>) {
    let a = Arc::clone(&arcTrieMap);
    let mut str= String::new();
    let mut reader = BufReader::new(&mut stream);
    reader.read_line(&mut str);
    //stream.read_to_string(&mut str).expect("TODO: panic message");
    println!("Received: {}", str);
    write!(stream,"hello {}", str).expect("TODO panic message");
    stream.flush().expect("TODO panic message");
}