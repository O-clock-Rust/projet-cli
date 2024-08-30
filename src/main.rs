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
