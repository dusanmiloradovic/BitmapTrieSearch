use super::*;

fn prepare_trie() -> Trie {
    let mut t = Trie::new();
    t.add_word("dragan", 0, 0);
    t.add_word("dragana", 1, 0);
    t.add_word("drni", 2, 0);
    t.add_word("dusan", 3, 0);
    t.add_word("nepar", 4, 0);
    t
}
#[test]
fn test_add() {
    let t = prepare_trie();
    let mut p = t
        .search("DR")
        .iter()
        .map(|x| x.word.clone())
        .collect::<Vec<String>>();
    p.sort();
    let t = vec![
        "DRAGAN".to_string(),
        "DRAGANA".to_string(),
        "DRNI".to_string(),
    ];
    assert_eq!(p, t);
}

#[test]
fn test_add_with_dictionary_index() {
    let t = prepare_trie();
    let mut p = t
        .search("DR")
        .iter()
        .map(|x| (x.word.clone(), x.entries.clone()))
        .collect::<Vec<(String, DictionaryMapEntry)>>();
    p.sort();
    let mut t = vec![
        (
            "DRAGAN".to_string(),
            DictionaryMapEntry {
                entries: vec![(0, 0)],
            },
        ),
        (
            "DRAGANA".to_string(),
            DictionaryMapEntry {
                entries: vec![(1, 0)],
            },
        ),
        (
            "DRNI".to_string(),
            DictionaryMapEntry {
                entries: vec![(2, 0)],
            },
        ),
    ];
    t.sort();
    assert_eq!(p, t);
}

#[test]
fn test_add_with_dictionary_index_with_duplicate_word() {
    let mut tr = prepare_trie();
    tr.add_word("dragan", 5, 0);
    tr.add_word("dragana", 7, 0);
    let mut p = tr
        .search("DR")
        .iter()
        .map(|x| (x.word.clone(), x.entries.clone()))
        .collect::<Vec<(String, DictionaryMapEntry)>>();
    p.sort();
    let mut t = vec![
        (
            "DRAGAN".to_string(),
            DictionaryMapEntry {
                entries: vec![(0, 0), (5, 0)], // not adding to dictionary entry, why??
            },
        ),
        (
            "DRAGANA".to_string(),
            DictionaryMapEntry {
                entries: vec![(1, 0), (7, 0)],
            },
        ),
        (
            "DRNI".to_string(),
            DictionaryMapEntry {
                entries: vec![(2, 0)],
            },
        ),
    ];
    t.sort();
    println!("{:?}", tr);
    assert_eq!(p, t);
}

#[test]
fn test_word_with_parent() {
    let mut tr = prepare_trie();
    tr.add_word("dragan miocinovic", 5, 0);
    //println!("{:#?}", tr);
    let p = tr
        .search("DRAGAN MIOC")
        .iter()
        .map(|x| x.word.clone())
        .collect::<Vec<String>>();
    let t = vec!["DRAGAN MIOCINOVIC".to_string()];

    assert_eq!(p, t);
}
#[test]
fn should_be_empty() {
    let tr = prepare_trie();
    let p = tr
        .search("dusana")
        .iter()
        .map(|x| x.word.clone())
        .collect::<Vec<String>>();
    let t: Vec<String> = vec![];
    assert_eq!(p, t);
}
