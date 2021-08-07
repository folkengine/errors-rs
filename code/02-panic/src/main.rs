use std::io::Write; // needed for write!()

fn hello(filename: &std::path::Path) {
    let file = std::fs::File::create(filename).unwrap();
    write!(&file, "Hello world!!\n").expect("Error writing to file.");
}

fn main() {
    hello(std::path::Path::new("numbers.txt"));
}