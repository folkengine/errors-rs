use std::io::BufRead; // needed for lines()

pub type BoxError = std::boxed::Box<dyn
    std::error::Error   // must implement Error to satisfy ?
    + std::marker::Send // needed for threads
    + std::marker::Sync // needed for threads
>;

fn count(filename: &std::path::Path) -> Result<i32, BoxError> {
    let file = std::fs::File::open(filename)?;
    let file = std::io::BufReader::new(file);
    let mut sum: i32 = 0;
    for line in file.lines() {
        sum += line?.parse::<i32>()?;
    }
    Ok(sum)
}

fn main() -> Result<(), BoxError> {
    let sum = count(std::path::Path::new("numbers.txt"))?;
    println!("The sum is: {}", sum);
    Ok(())
}