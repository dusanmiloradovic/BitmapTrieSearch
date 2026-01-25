use super::*;

fn prepare_trie()->Trie{
   let mut t =  Trie::new();
    t.add_word("dragan", 0, 0);
    t.add_word("dragana",1,0);
    t.add_word("drni",2,0);
    t.add_word("dusan",3,0);
    t.add_word("nepar",4,0);
    t
}
#[test]
fn test_add() {
    let t = prepare_trie();
    let p = t.search("DR").iter().map(|x| x.word.clone()).collect::<Vec<String>>().sort();
    let t = vec!["DRAGAN".to_string(), "DRAGANA".to_string(), "DRNI".to_string()].sort();
    assert_eq!(p, t);
}

#[test]
fn test_word_with_parent() {
    let mut t = prepare_trie();
    t.add_word("dragan miocinovic",5,0);
    let p = t.search("DRAGAN M").iter().map(|x| x.word.clone()).collect::<Vec<String>>();
    let t = vec!["DRAGAN MIOCINOVIC".to_string()];
    assert_eq!(p, t);
}
