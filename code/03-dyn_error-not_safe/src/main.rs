use std::io::BufRead; // needed for lines()

fn count(filename: &std::path::Path) -> Result<i32, std::boxed::Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filename)?;
    let file = std::io::BufReader::new(file);
    let mut sum: i32 = 0;
    for line in file.lines() {
        sum += line?.parse::<i32>()?;
    }
    Ok(sum)
}

fn main() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    let sum = count(std::path::Path::new("numbers.txt"))?;
    println!("The sum is: {}", sum);
    Ok(())
}