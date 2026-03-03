use std::{
    fs::File,
    path::Path,
};

use crate::is_file_exists;



fn file_ident(_file: &File) -> Result<(), String> {
    Ok(())
}

pub fn file_analyse(path_file: Option<String>) -> Result<(), String> {
    match path_file {
        Some(pf) => {
            println!("file {pf}");
            match is_file_exists(Path::new(&pf)) {
                Ok(file) => file_ident(&file),
                Err(msg) => Err(msg),
            }
        }
        None => Err("Need path".to_string()),
    }
}
