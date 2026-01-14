use bitmap_trie::command::{Command, Response, SearchCommand};
use bitmap_trie::listener;
use bitmap_trie::trie;
use serde_json;

fn main() {
    // let mut t = trie::Trie::new();
    // t.add_word("dusana");
    // t.add_word("dusan");
    // t.add_word("dejan");
    // t.add_word("dragan");
    // t.add_word("dragana");
    // t.add_word("draganovic");
    // t.add_word("dulitl");
    // t.add_word("srecko");
    // t.add_word("sreten");
    // t.add_word("sretenka");
    // t.add_word("petar");
    // t.add_word("vepar");
    // t.add_word("nepar");
    // let p = t.search("SR");
    // print!("{:#?}\n", p);
    // let c = Command::Search(SearchCommand {
    //     trie_id: "1".to_string(),
    //     term: "SR".to_string(),
    // });
    // let s = serde_json::to_string(&c).unwrap();
    // println!("{}\n", s);
    listener::listen().expect("TODO: panic message");
}
