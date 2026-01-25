use bitmap_trie::command::{AddWordCommand, Command, Response, SearchCommand};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};

fn add_word_client(stream: &TcpStream, trie_id: &str, word: &str, dictionary_index:u32, dictionary_attribute:u8) {
    let reader = BufReader::new(stream);
    let writer = BufWriter::new(stream);
    let mut serializer = serde_json::Serializer::new(writer);
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let c = Command::AddWord(AddWordCommand {
        trie_id: trie_id.to_string(),
        word: word.to_string(),
        dictionary: "".to_string(),
        dictionary_index,
        dictionary_attribute,
    });
    c.serialize(&mut serializer).unwrap();
    serializer.into_inner().flush().unwrap();
    Response::deserialize(&mut deserializer).unwrap();
}

fn search_term_client(stream: &TcpStream, trie_id:&str, term:&str){
    let reader = BufReader::new(stream);
    let writer = BufWriter::new(stream);
    let mut serializer = serde_json::Serializer::new(writer);
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let c = Command::Search(SearchCommand{trie_id:trie_id.to_string(),term:term.to_string()});
    c.serialize(&mut serializer).unwrap();
    serializer.into_inner().flush().unwrap();
    let r = Response::deserialize(&mut deserializer).unwrap();
    println!("{:?}",r);
}

fn main() {
    let  stream = TcpStream::connect("127.0.0.1:4444").unwrap();
    add_word_client(&stream, "1", "dusana",0,0);
    add_word_client(&stream,"1","dusan",1,0);
    add_word_client(&stream,"1","dejan",2,0);
    add_word_client(&stream,"1","dragan",3,0);
    add_word_client(&stream,"1","dragana",4,0);
    add_word_client(&stream,"1","draganovic",5,0);
    add_word_client(&stream,"1","dulitl",6,0);
    add_word_client(&stream,"1","srecko",7,0);
    add_word_client(&stream,"1","sreten",8,0);
    add_word_client(&stream,"1","sretenka",9,0);
    add_word_client(&stream,"1","petar",10,0);
    add_word_client(&stream,"1","vepar",11,0);
    add_word_client(&stream,"1","nepar",12,0);
    add_word_client(&stream,"1","dragan milutinovic milutinac",13,0);
    search_term_client(&stream,"1","DRAGAN MI");

}
