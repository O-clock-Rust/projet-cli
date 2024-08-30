use std::{fmt, io};

/// Description des différents types d'erreur
#[derive(Debug)]
pub enum AppErrorKind {
    InvalidData,
    NoArgs,
    NotFound,
    Unknown,
}

impl fmt::Display for AppErrorKind {
    // Affiche une description textuelle des types d'erreur
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl From<io::ErrorKind> for AppErrorKind {
    fn from(error_kind: io::ErrorKind) -> Self {
        match error_kind {
            io::ErrorKind::InvalidData => AppErrorKind::InvalidData,
            io::ErrorKind::NotFound => AppErrorKind::NotFound,
            _ => AppErrorKind::Unknown,
        }
    }
}

impl AppErrorKind {
    /// Convertit chaque variante en chaînes de caractères
    /// `&'static` → durée de vie = durée du programme
    fn as_str(&self) -> &'static str {
        use AppErrorKind::*;

        // déréférence `self`
        match *self {
            InvalidData => "le format du fichier n'est pas pris en compte",
            NoArgs => {
                "vous devez fournir un chemin de fichier à lire → `$ cargo run -- sample.txt`"
            }
            NotFound => "le fichier demandé est introuvable !",
            Unknown => "erreur inconnue",
        }
    }
}

/// AppError
// #[derive(Debug)]
pub struct AppError {
    pub kind: AppErrorKind,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_msg = &self.kind.to_string();

        write!(f, "{}", err_msg)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self {
            kind: error.kind().into(),
        }
    }
}

impl From<AppErrorKind> for AppError {
    fn from(kind: AppErrorKind) -> Self {
        Self { kind }
    }
}
