use std::{env, process};

use error::Error;
use file::File;

mod error;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    let _ = read_file(file_path.to_string());
}

fn read_file(file_path: String) -> Result<(), Error> {
    let file = File::new(file_path.to_string());

    match file.read() {
        Ok(contents) => {
            println!("{}", contents);
            Ok(())
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work_if_file_exists_and_is_not_empty() {
        assert!(read_file("example.txt".to_string()).is_ok())
    }

    #[test]
    fn it_should_not_work_if_file_is_empty() {
        assert!(read_file("empty.txt".to_string()).is_err())
    }

    #[test]
    fn it_should_not_work_if_file_path_is_empty() {
        assert!(read_file("".to_string()).is_err())
    }

    #[test]
    fn it_should_not_work_if_file_path_does_not_exist() {
        assert!(read_file("hello".to_string()).is_err())
    }
}
