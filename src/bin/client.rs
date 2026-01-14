use bitmap_trie::command::{AddWordCommand, Command, Response};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};

fn add_word_client(stream: &TcpStream, trie_id: &str, word: &str) {
    let reader = BufReader::new(stream);
    let writer = BufWriter::new(stream);
    let mut serializer = serde_json::Serializer::new(writer);
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let c = Command::AddWord(AddWordCommand {
        trie_id: trie_id.to_string(),
        word: word.to_string(),
        dictionary: "".to_string(),
    });
    c.serialize(&mut serializer).unwrap();
    let res = Response::deserialize(&mut deserializer).unwrap();
    println!("{:?}", res);
}

fn main() {
    let  stream = TcpStream::connect("127.0.0.1:4444").unwrap();
    add_word_client(&stream, "1", "dusana");
    add_word_client(&stream,"1","dusan");
    add_word_client(&stream,"1","dejan");
    add_word_client(&stream,"1","dragan");
    add_word_client(&stream,"1","dragana");
    add_word_client(&stream,"1","draganovic");
    add_word_client(&stream,"1","dulitl");
    add_word_client(&stream,"1","srecko");
    add_word_client(&stream,"1","sreten");
    add_word_client(&stream,"1","sretenka");
    add_word_client(&stream,"1","petar");
    add_word_client(&stream,"1","vepar");
    add_word_client(&stream,"1","nepar");

}
