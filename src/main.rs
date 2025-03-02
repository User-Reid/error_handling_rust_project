use std::io::stdin;
use std::fs;
use std::process::exit;

fn write_to_file() {
    let mut x: String = String::new();
    let mut y: String = String::new();
    println!("What file would you like to write to?");
    stdin().read_line(&mut x)?;
    print!("What would you like to write to the file?");
    stdin().read_line(&mut y)?;

    fs::write(x, y)?;

    Ok(x)
}

fn main() {
    let x = match write_to_file() {
        Ok(some) => println!("Successfully wrote to the file {some}"),
        Err(error) => {
            eprintln!("There was an error: {error}");
            exit(1);
    }
};
}
