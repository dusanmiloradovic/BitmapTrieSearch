mod trie;
mod encoding;

fn main() {
    let mut t = trie::Trie::new();
    t.add_word("dusana");
    t.add_word("dusan");
    t.add_word("dejan");
    t.add_word("dragan");
    t.add_word("dragana");
    t.add_word("draganovic");
    t.add_word("dulitl");
    t.add_word("srecko");
    t.add_word("sreten");
    t.add_word("sretenka");
    t.add_word("petar");
    t.add_word("vepar");
    t.add_word("nepar");
    let p = t.search("SR");
    print!("{:#?}", p);
}
