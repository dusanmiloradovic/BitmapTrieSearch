use std::io::{Read, Write};
use std::net::TcpStream;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:4444").unwrap();
    stream.write_all(b"dusan\n").unwrap();
    let mut str= String::new();
    stream.read_to_string(&mut str).unwrap();
    println!("Received from server {}", str);
}