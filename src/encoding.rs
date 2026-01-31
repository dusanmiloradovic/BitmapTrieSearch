use std::fmt::Debug;
use std::sync::OnceLock;

// This is a 64 char set that will be supported for trie search
// 64 for bitmap maps to u64
const ASCII_CHARS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_";



pub trait Encoding:Debug{
    fn idx(&self,c:char)->u8;
    fn decode(&self,idx:u8)->char;
}

pub trait Translate{
    fn translate_encode(&self, str: &str) -> String;
    //we can't always just use encode, because se might want to transliterate one char to multiple trie chars (for example Š to SH)
    fn translate_decode(&self, original_str: &str, str: &str) -> &str;
    // Because we lose the information when encoding, we need an original string to compare, if match we return the part of original string that was encoded
}

#[derive(Debug)]
pub struct AsciiEncoding;

impl Encoding for AsciiEncoding{
    fn idx(&self,c:char)->u8{
        let b = c.to_uppercase();
        let k = b.to_string();
        match ASCII_CHARS.find(&k){
            Some(u) => u as u8,
            None=>63 // map all non-mapped chars to "_"
        }
    }

    fn decode(&self,idx: u8) -> char {
        ASCII_CHARS.chars().nth(idx as usize).unwrap()
    }
}

static ENCODING: OnceLock<Box<dyn Encoding + Send + Sync>> = OnceLock::new();

// Initialize once at startup, default is ascii
pub fn init_encoding(encoding: Box<dyn Encoding + Send + Sync>) {
    ENCODING.set(encoding).expect("Encoding already initialized");
}

// Access from anywhere
pub fn get_encoding() -> &'static dyn Encoding {
    match ENCODING.get() {
        None => {
            init_encoding(Box::new(AsciiEncoding{}));
            ENCODING.get().unwrap().as_ref()
        },
        Some(encoding) =>encoding.as_ref()
    }
}

// Helper to get idx directly
pub fn idx(c: char) -> u8 {
    get_encoding().idx(c)
}

pub fn decode(idx: u8) -> char {
    get_encoding().decode(idx)
}
#[cfg(test)]
mod test{
    use crate::encoding::idx;
    use crate::encoding::ASCII_CHARS;

    #[test]
    fn char_eq(){
        assert_eq!(idx('c'), ASCII_CHARS.find("C").unwrap() as u8);
    }
    #[test]
    fn non_existing(){
        assert_eq!(idx('{'), ASCII_CHARS.find("_").unwrap() as u8);
    }
}