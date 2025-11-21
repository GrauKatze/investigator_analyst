#![warn(missing_docs)]

//! Analyse metadata files

pub mod configuration {

    use log::debug;

    use crate::{fl_analyse, manual};
    pub enum ConfigType {
        #[allow(missing_docs)]
        Help,
        #[allow(missing_docs)]
        Version,
        FileAnalyse(Option<String>),
        FilesAnalyse(Option<Vec<String>>),
        Manual(Option<Vec<String>>),
    }

    fn write_help() {
        println!(env!("CARGO_PKG_NAME"));
        println!("version: {}", env!("CARGO_PKG_VERSION"));
        println!(env!("CARGO_PKG_DESCRIPTION"));
        println!("\nUsage: {} <key>\n", env!("CARGO_PKG_NAME"));
        println!("KEYS:");
        println!("{:3} | {:25} {}", "-h", "--help", "this text");
        println!("{:3} | {:25} {}", "-v", "--version", "version");
        println!("{:3} | {:25} {}", "-m", "--manual <OPT>", "manual");
        println!("{:3} | {:25} {}", "-f", "--file <PATH>", "File analyse")
    }

    fn args_pars(args: Vec<String>) -> Result<ConfigType, String> {
        let arg = args.get(0);
        debug!("{:?}", &arg);
        match arg {
            Some(p) => {
                match p.as_str() {
                    "-h" | "--help" | "h" | "help" => Ok(ConfigType::Help),
                    "-v" | "--version" | "v" | "version" => Ok(ConfigType::Version),

                    //FIXME: maybe i can use only vec<string>?
                    "-m" | "--manual" | "m" | "manual" => match args.get(1..) {
                        Some(conf) => Ok(ConfigType::Manual(Some(conf.to_vec()))),
                        None => Ok(ConfigType::Manual(None)),
                    },
                    "-f" | "--file" | "f" | "file" => {
                        let file = args.get(1);
                        Ok(ConfigType::FileAnalyse(file.cloned()))
                    }
                    "-fs" | "--files" | "fs" | "files" => match args.get(1..) {
                        Some(pathes) => Ok(ConfigType::FilesAnalyse(Some(pathes.to_vec()))),
                        None => Ok(ConfigType::FilesAnalyse(None)),
                    },

                    _ => Err(format!("not found this arguments\n{{ {} }}", args.concat())),
                }
            }
            None => Err("need more argument".to_string()),
        }
    }

    /// Initial program configuration
    pub fn init(args: Vec<String>) -> Result<ConfigType, String> {
        debug!("{:?}", &args);
        match args_pars(args) {
            Ok(config_type) => Ok(config_type),
            Err(msg) => Err(msg),
        }
    }
    //TODO: add doc
    #[allow(missing_docs)]
    pub fn run(config_type: ConfigType) -> Result<(), String> {
        match config_type {
            ConfigType::Help => {
                write_help();
                Ok(())
            }
            ConfigType::Version => {
                println!("version: {}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
            ConfigType::FileAnalyse(path) => fl_analyse::file_analyse(path),
            ConfigType::FilesAnalyse(files) => fl_analyse::files_analyse(files),
            ConfigType::Manual(opt) => manual::run(opt),
        }
    }
}

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
        Ok(())
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
                        Err(msg) => return Err(msg),
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

        fl_analyse::files_analyse(pathes);
    }
}
