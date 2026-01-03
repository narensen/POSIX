use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("test.txt")?;
    let mut buffer = String::new();

    println!("Before read");
    file.read_to_string(&mut buffer)?;
    println!("After read");

    Ok(())
}
