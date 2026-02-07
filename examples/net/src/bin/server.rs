use netexample::listener;

fn main() {
    listener::listen().expect("Server failed to start");
}
