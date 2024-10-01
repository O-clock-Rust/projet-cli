use std::fs;

use crate::error::Error;

pub struct File(String);

impl File {
    pub fn new(path: String) -> Self {
        Self(path)
    }

    pub fn read(&self) -> Result<String, Error> {
        let content = fs::read_to_string(&self.0)?;

        if content.is_empty() {
            return Err(Error(String::from("le fichier est vide")));
        }

        Ok(content)
    }
}
