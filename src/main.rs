use crate::cmd::parse_args;
use std::path::Path;
use rpassword::{prompt_password_stdout};
use blake2::{Blake2b, Digest};

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

    // get password digest
    let pass_digest = Blake2b::digest(password.as_ref());

    // get login digest
    let login_digest = Blake2b::digest((&args.login).as_ref());

    // print login and password digest
    println!("login digest: {:x}", login_digest);
    println!("password digest: {:x}", pass_digest);
}
