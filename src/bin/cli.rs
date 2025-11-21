fn main() {
    println!(env!("CARGO_PKG_NAME"));
    println!("===START===");

    match match analyst::configuration::init(Some(std::env::args().skip(1).collect())) {
        Ok(cnf) => analyst::configuration::run(cnf),
        Err(msg) => Err(msg),
    } {
        Ok(()) => (),
        Err(msg) => println!("{}", msg),
    }
    println!("===END===");
}
