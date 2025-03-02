
use std::io;
use std::fs;
use std::io::Error;
use std::process::exit;

fn write_to_file() -> Result<String, Error> {
    let input = io::stdin();

    println!("What file would you like to write to?");
    let mut x: String = String::new();
    input.read_line(&mut x)?;

    println!("What would you like to write to the file?");
    let mut y: String = String::new();
    input.read_line(&mut y)?;

    fs::write(&x.trim(), y.trim())?;

    Ok(x)
}

fn main() {
    match write_to_file() {
        Ok(some) => println!("Successfully wrote to file {some}"),
        Err(error) => {
            eprintln!("There was an error {error}");
            exit(1)
        }
    }
}