mod trie;
mod encoding;

fn main() {
    let mut t = trie::Trie::new();
    t.add_word("dusan");
    t.add_word("dejan");
    t.add_word("dragan");
    t.add_word("dragana");
    t.add_word("draganovic");
    t.add_word("dulitl");
    print!("{:?}",t);
}
