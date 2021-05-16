use crate::cmd::parse_args;
use std::path::Path;
use rpassword::{prompt_password_stdout};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

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

    let mut hasher = Sha256::new();

    // get password digest
    hasher.input_str(&*password);
    let pass_digest = hasher.result_str();

    // reset hasher for using with new value
    hasher.reset();

    // get login digest
    let login = &args.login;
    hasher.input_str(&*login);
    let login_digest = hasher.result_str();

    // print login and password digest
    println!("login digest: {}", login_digest);
    println!("password digest: {}", pass_digest);

}
