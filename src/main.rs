use std::{env, process};

use error::Error;
use file::File;

mod error;
mod file;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    let file = File::new(file_path.to_string());

    file.read()
}
