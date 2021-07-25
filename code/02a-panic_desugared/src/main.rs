
use std::io::Write; // needed for write!()

fn hello(filename: &std::path::Path) {
    let file = match std::fs::File::create(filename) {
        Ok(f) => f,
        Err(_) => panic!()
    };
    if let Err(_) = write!(&file, "Hello world!\n") {
        panic!("Error writing to file.");
    }
}

fn main() {
    hello(std::path::Path::new("hello.txt"));
}