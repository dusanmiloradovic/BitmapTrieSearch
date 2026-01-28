// This is a 64 char set that will be supported for trie search
// 64 for bitmap maps to u64
pub const CHARS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_";


// TODO add the optional CHARS (dictionary), for example only Cyrillic    
pub fn idx(c:char)->u8{
    let b = c.to_uppercase();
    let k = b.to_string();
    match CHARS.find(&k){
        Some(u) => u as u8,
        None=>63 // map all non-mapped chars to "_"
    }
}

#[cfg(test)]
mod test{
    use crate::encoding::idx;
    use crate::encoding::CHARS;

    #[test]
    fn char_eq(){
        assert_eq!(idx('c'), CHARS.find("C").unwrap() as u8);
    }
    #[test]
    fn non_existing(){
        assert_eq!(idx('{'), CHARS.find("_").unwrap() as u8);
    }
}