/// Étapes :
///
///   1. récupérer les arguments de la commande
///      → `cargo run -- README.md`
///
///   2. lire le fichier README.md
///
///   3. afficher le résultat
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
}
