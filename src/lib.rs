#![warn(missing_docs)]

//! Analyse metadata files

pub mod configuration;

mod manual {
    pub fn run(_opt: Option<Vec<String>>) -> Result<(), String> {
        println!("MANUAL");
        Ok(())
    }
}

mod fl_analyse {
    use std::path::Path;

    ///Try to ident file
    fn file_ident(_fl: &Path) -> Result<(), String> {
        match _fl.exists() {
            true => Ok(()),
            false => Err("not correct path".to_string()),
        }
    }

    pub fn file_analyse(path_file: Option<String>) -> Result<(), String> {
        match path_file {
            Some(pf) => {
                println!("{pf}");
                match file_ident(Path::new(&pf)) {
                    Ok(()) => Ok(()),
                    Err(msg) => Err(msg),
                }
            }
            None => Err("Need path".to_string()),
        }
    }

    pub fn files_analyse(path_files: Option<Vec<String>>) -> Result<(), String> {
        println!("files analyse");
        match path_files {
            Some(pathes) => {
                for i in pathes {
                    match file_analyse(Some(i)) {
                        Ok(()) => (),
                        Err(msg) => println!("{msg}"),
                    }
                }
                Ok(())
            }
            None => Err("msg".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fl_analyse;

    #[test]
    fn files_analyse_with_empty_path() {
        let pathes = Some(vec!["one".to_string(), "".to_string(), "three".to_string()]);

        fl_analyse::files_analyse(pathes).unwrap();
    }
}
