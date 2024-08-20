use std::fs;

use crate::error::Error;

pub struct File(String);

impl File {
    pub fn new(path: String) -> Self {
        Self(path)
    }

    pub fn read(&self) -> Result<(), Error> {
        let contents = fs::read_to_string(&self.0)?;

        if contents.is_empty() {
            return Err(Error("File looks empty!".to_string()));
        }

        println!("{}", contents);

        Ok(())
    }
}
