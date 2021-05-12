use crate::cmd::parse_args;
use std::path::Path;

pub mod cmd;

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    println!("{:#?}", args);
    if Path::new(&args.file).exists() {
        println!("file exists");
    } else {
        println!("file not exists");
    }
}
