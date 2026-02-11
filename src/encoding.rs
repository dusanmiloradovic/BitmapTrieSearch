use std::fmt::Debug;
use std::sync::OnceLock;

// This is a 64 char set that will be supported for trie search
// 64 for bitmap maps to u64
const ASCII_CHARS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_";

pub trait Encoding: Debug {
    fn idx(&self, c: char) -> u8;
    fn decode(&self, idx: u8) -> char;
    // Because we lose the information when encoding, we need an original string to compare, if match we return the part of the original string that was encoded
    fn translate_encode(&self, str: &str) -> String;
    //we can't always just use encode, because se might want to transliterate one char to multiple trie chars (for example Š to SH)
    fn translate_decode<'a>(&self, original_str: &'a str, ind: usize, str: &str) -> &'a str;
    fn get_separator(&self) -> char {
        ' '
    }
}

#[derive(Debug)]
pub struct AsciiEncoding;

impl Encoding for AsciiEncoding {
    fn idx(&self, c: char) -> u8 {
        let b = c.to_uppercase();
        let k = b.to_string();
        match ASCII_CHARS.find(&k) {
            Some(u) => u as u8,
            None => 63, // map all non-mapped chars to "_"
        }
    }

    fn decode(&self, idx: u8) -> char {
        ASCII_CHARS.chars().nth(idx as usize).unwrap()
    }

    // TODO use graphemes for both encode and decode
    // Maybe not for decode, instead of passing the string from trie, we could pass the start index and no of bytes(and return just the byte slice)
    fn translate_encode(&self, str: &str) -> String {
        let mut ret = String::new();
        for c in str.chars() {
            let uc =c.to_uppercase().next().unwrap();
            if ASCII_CHARS.find(&uc.to_string()).is_none() {
                ret.push('_');
            }else{
                ret.push(uc);
            }

        }
        ret
    }

    // this will just get the slice in the future, I will not pass the str, instead the length of original(encoded) string in bytes
    fn translate_decode<'a>(&self, original_str: &'a str, ind: usize, str: &str) -> &'a str {
        // no need to compare, relying on the fact that translate_encode was done correctly
        // this is correct only if the encoded string has the same number of chars as the original
        if let Some(s) = original_str.get(ind..ind + str.len()) {
            s
        } else {
            print!("translate_decode: index out of bounds: ind={}, str.len={}, original_str.len={}\n", ind, str.len(), original_str.len());
            print!(
            "original_str: '{}', str: '{}'\n", original_str,str);
            print!("**********************************\n");
            ""
        }
    }
}

static ENCODING: OnceLock<Box<dyn Encoding + Send + Sync>> = OnceLock::new();

// Initialize once at startup, default is ascii
pub fn init_encoding(encoding: Box<dyn Encoding + Send + Sync>) {
    ENCODING
        .set(encoding)
        .expect("Encoding already initialized");
}

// Access from anywhere
pub fn get_encoding() -> &'static dyn Encoding {
    match ENCODING.get() {
        None => {
            init_encoding(Box::new(AsciiEncoding {}));
            ENCODING.get().unwrap().as_ref()
        }
        Some(encoding) => encoding.as_ref(),
    }
}

// Helper to get idx directly
pub fn idx(c: char) -> u8 {
    get_encoding().idx(c)
}

pub fn decode(idx: u8) -> char {
    get_encoding().decode(idx)
}

pub fn translate_encode(str: &str) -> String {
    get_encoding().translate_encode(str)
}

pub fn translate_decode<'a>(original_str: &'a str, ind: usize, str: &str) -> &'a str {
    get_encoding().translate_decode(original_str, ind, str)
}

#[cfg(test)]
mod test {
    use crate::encoding::idx;
    use crate::encoding::ASCII_CHARS;

    #[test]
    fn char_eq() {
        assert_eq!(idx('c'), ASCII_CHARS.find("C").unwrap() as u8);
    }
    #[test]
    fn non_existing() {
        assert_eq!(idx('{'), ASCII_CHARS.find("_").unwrap() as u8);
    }
}
