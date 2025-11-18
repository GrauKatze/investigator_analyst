#![warn(missing_docs)]

//! Analyse metadata files

/// modile for start analys
pub mod file_identification {
    use std::{fs::Metadata, path::Path};

    /// to know file
    pub fn _file_ident(file_path: String) -> Result<Metadata, String> {
        println!("{}", &file_path);
        if Path::exists(Path::new(&file_path)) {
            Ok(_take_file_metadata(&file_path).unwrap())
        } else {
            Err(format!("path is not valid, path: \"{}\"", file_path))
        }
    }

    fn _take_file_metadata(_file_path: &String) -> Result<Metadata, String> {
        Ok(Path::new(&_file_path).metadata().unwrap())
    }

    ///Return executble format
    fn _check_file_executble(_file_path: &String) -> Result<String, String> {
        Ok("x86".to_string())
    }
}
