//! Analyse metadata files

use std::{fs::File, path::Path};

pub mod configuration;

mod fl_analyse;

/// Проверка на существование файла
fn is_file_exists(fl: &Path) -> Result<File, String> {
    match fl.exists() {
        true => Ok(File::open(fl).unwrap()),
        false => Err("not correct path".to_string()),
    }
}

#[cfg(test)]
mod tests {
}
