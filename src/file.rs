use std::fs;

use crate::error::AppError;

pub struct File {
    pub path: String,
    content: Option<String>,
}

impl File {
    pub fn new(path: String) -> Self {
        File {
            path,
            content: None,
        }
    }

    pub fn read(&mut self) -> Result<String, AppError> {
        match fs::read_to_string(&self.path) {
            Ok(file_content) => {
                self.content = Some(file_content.clone());
                Ok(file_content)
            }
            Err(e) => Err(AppError::from(e)),
        }
    }

    pub fn print(&mut self) {
        match self.read() {
            Ok(content) => {
                let result = if content.is_empty() {
                    String::from("Le fichier est vide.")
                } else {
                    content
                };

                println!("{}", result);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
