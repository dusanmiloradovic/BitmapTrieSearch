use super::*;

fn prepare_trie()->Trie{
   let mut t =  Trie::new();
    t.add_word("dragan");
    t.add_word("dragana");
    t.add_word("drni");
    t.add_word("dusan");
    t.add_word("nepar");
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
    t.add_word("dragan miocinovic");
    let p = t.search("DRAGAN M").iter().map(|x| x.word.clone()).collect::<Vec<String>>();
    let t = vec!["DRAGAN MIOCINOVIC".to_string()];
    assert_eq!(p, t);
}
