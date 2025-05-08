use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use serde::Deserialize;
use serde::Serialize;

pub struct JsonIo<'a> {
    file_name: &'a str,
}

impl<'a> JsonIo<'a> {
    pub fn new(file_name: &'a str) -> Self {
        Self { file_name }
    }

    pub fn write_json<T: Serialize>(&self, a_struct: &T) -> Result<(), Box<dyn Error>> {
        // Serialize to JSON string
        let json_string = serde_json::to_string_pretty(a_struct)?; // Use `to_string` for compact output
    
        // Create or overwrite the file
        let mut file = File::create(self.file_name)?;
    
        // Write the JSON string to the file
        file.write_all(json_string.as_bytes())?;
    
        Ok(())
    }

    pub fn read_json<T: for<'b> Deserialize<'b>>(&self) -> Result<T, Box<dyn Error>> {
        // Open the file
        let mut file = File::open(self.file_name)?;
    
        // Read the file contents into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
    
        let deserialized_json = serde_json::from_str::<T>(&contents)?;
        // Deserialize the JSON string into a Person struct
    
        Ok(deserialized_json)
    }
}