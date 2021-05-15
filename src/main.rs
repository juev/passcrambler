use crate::cmd::parse_args;
use std::path::Path;
use aes::cipher::generic_array::GenericArray;
use rpassword::{prompt_password_stdout};
use sha256::digest;

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
    if !Path::new(&args.file).exists() {
        println!("file not exists");
    }

    let password = prompt_password_stdout("Type password: ").unwrap();
    println!("login digest: {}", digest(&args.login));
    println!("password digest: {}", digest(password));
    // let key = GenericArray::from_slice(&[0u8; 16]);
}
