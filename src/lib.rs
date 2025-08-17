#![warn(missing_docs)]

//! Analyse metadata files

/// modile for start analys
pub mod file_identification {
    use std::path::Path;

    /// to know file
    pub fn _file_ident(file_path: String) -> Result<(), String> {
        if Path::exists(Path::new(&file_path)) {
            _take_file_metadata(&file_path).unwrap();
            _check_file_executble(&file_path).unwrap();
            Ok(())
        } else {
            Err(format!("path is not valid, path: \"{}\"", file_path))
        }
    }

    fn _take_file_metadata(_file_path: &String) -> Result<(), String> {
        Ok(())
    }

    ///Return executble format
    fn _check_file_executble(_file_path: &String) -> Result<String, String> {
        Ok("x86".to_string())
    }
}
