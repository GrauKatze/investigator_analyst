use log::debug;

pub enum ConfigType {
    #[allow(missing_docs)]
    Help,
    #[allow(missing_docs)]
    Version,
}

fn write_help() {
    println!(env!("CARGO_PKG_NAME"));
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    println!(env!("CARGO_PKG_DESCRIPTION"));
    println!("\nUsage: {} <key>\n", env!("CARGO_PKG_NAME"));
    println!("KEYS:");
    println!("{:3} | {:25} {}", "-h", "--help", "this text");
    println!("{:3} | {:25} {}", "-v", "--version", "version");
    println!("{:3} | {:25} {}", "-f", "--file <PATH>", "File analyse");
}

fn args_pars(args: Vec<String>) -> Result<ConfigType, String> {
    let arg = args.get(0);
    debug!("{:?}", &arg);
    match arg {
        Some(p) => match p.as_str() {
            "-h" | "--help" | "h" | "help" => Ok(ConfigType::Help),
            "-v" | "--version" | "v" | "version" => Ok(ConfigType::Version),

            _ => Err(format!("not found this arguments\n{{ {} }}", args.concat())),
        },
        None => Err("need more argument".to_string()),
    }
}

/// Initial program configuration
pub fn init(args: Option<Vec<String>>) -> Result<ConfigType, String> {
    debug!("{:?}", &args);
    match args {
        Some(args) => match args_pars(args) {
            Ok(config_type) => Ok(config_type),
            Err(msg) => Err(msg),
        },
        None => Err("()".to_string()),
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
    }
}
