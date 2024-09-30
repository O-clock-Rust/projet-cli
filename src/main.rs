use std::{env, process};

use file::File;

mod error;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:#?}", args);

    if args.len() < 2 {
        println!(
            "Erreur : vous devez fournir un chemin de fichier à lire → `$ cargo run -- sample.txt`",
        );
        process::exit(1);
    }

    let file_path = &args[1];
    read_file(file_path.to_string());
}

fn read_file(file_path: String) {
    let file = File::new(file_path);

    // récupérer le contenu
    match file.read() {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Erreur : {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_work() {
        let file = File::new(String::from("example.txt"));
        assert!(file.read().is_ok())
    }

    #[test]
    fn it_should_not_work_if_file_not_exist() {
        let file = File::new(String::from("not_exist.txt"));
        assert!(file.read().is_err())
    }

    #[test]
    fn it_should_not_work_if_file_is_empty() {
        let file = File::new(String::from("empty.txt"));
        assert!(file.read().is_err())
    }
}
