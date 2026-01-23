use bitmap_trie::listener;

fn main() {
    listener::listen().expect("Server failed to start");
}
