// This is a 64 char set that will be supported for trie search
// 64 for bitmap maps to u64
const chars: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_";

pub fn idx(c:char)->u8{
    let b = c.to_uppercase();
    let k = b.to_string();
    match chars.find(&k){
        Some(u) => u as u8,
        None=>63 // map all non-mapped chars to "_"
    }
}

#[cfg(test)]
mod test{
    use crate::encoding::idx;
    use crate::encoding::chars;

    #[test]
    fn char_eq(){
        assert_eq!(idx('c'), chars.find("C").unwrap() as u8);
    }
    #[test]
    fn non_existing(){
        assert_eq!(idx('{'), chars.find("_").unwrap() as u8);
    }
}