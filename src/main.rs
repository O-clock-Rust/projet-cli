use error::{AppError, AppErrorKind};
use file::File;
/// Étapes :
///
///   1. récupérer les arguments de la commande
///      → `cargo run -- README.md`
///
///   2. lire le fichier README.md
///
///   3. afficher le résultat
use std::env;

mod error;
mod file;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    // println!("{:#?}", args);

    if args.len() < 2 {
        return Err(AppError::from(AppErrorKind::NoArgs).to_string());
    }

    let path = &args[1];
    let mut file = File::new(path.to_string());

    file.print();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_file_ok() {
        let mut file = File::new(String::from("README.md"));
        assert!(file.read().is_ok())
    }

    #[test]
    fn if_file_is_not_found() {
        let mut file = File::new(String::from("FAKE.txt"));
        assert!(file.read().is_err())
    }

    #[test]
    fn if_file_is_a_dir() {
        let mut file = File::new(String::from("src"));
        assert!(file.read().is_err())
    }

    #[test]
    fn if_file_is_invalid() {
        let mut file = File::new(String::from("rust.png"));
        assert!(file.read().is_err())
    }

    #[test]
    fn if_file_is_empty() {
        let mut file = File::new(String::from("empty_file.txt"));
        if let Ok(content) = file.read() {
            assert!(content.is_empty())
        }
    }

    #[test]
    fn if_file_is_not_empty() {
        let mut file = File::new(String::from("README.md"));
        if let Ok(content) = file.read() {
            assert!(content.contains('a'))
        }
    }
}
