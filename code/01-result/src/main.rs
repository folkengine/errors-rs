use std::io::Write; // needed for write!()

fn hello(filename: &std::path::Path) -> Result<(), std::io::Error> {
    let file = std::fs::File::create(filename)?; // the question mark
    write!(&file, "Hello world!\n")
}

fn main() -> Result<(), std::io::Error> {
    hello(std::path::Path::new("hello.txt"))
}