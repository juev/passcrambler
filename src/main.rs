use crate::cmd::parse_args;

use rpassword::prompt_password_stdout;
use std::path::Path;

use crypto::aes::{cbc_encryptor, KeySize};
use crypto::blockmodes::{PkcsPadding, NoPadding};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};

use crypto::digest::Digest;
use crypto::sha3::Sha3;
use crypto::symmetriccipher::SymmetricCipherError;

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

    let mut hasher = Sha3::sha3_256();
    // get password digest
    hasher.input(password.as_ref());

    let pass_digest = hasher.result_str();

    // get login digest
    hasher.reset();
    hasher.input((&args.login).as_ref());

    let login_digest = hasher.result_str();

    // print login and password digest
    println!("login digest: {}", login_digest);
    println!("password digest: {}", pass_digest);

    let result = aes_encrypt(
        b"Hello world, hello world, hello world",
        pass_digest.as_bytes(),
        login_digest.as_bytes(),
    );

    println!("{:?}", result);
}

pub fn aes_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut encryptor = cbc_encryptor(
        KeySize::KeySize256,
        key,
        iv,
        PkcsPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut buffer = [0; 16];
    let mut read_buffer = RefReadBuffer::new(data);
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    loop {
        // thread 'main' panicked at 'attempt to subtract with overflow'
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);
        match result {
            Ok(BufferResult::BufferUnderflow) => break,
            Ok(BufferResult::BufferOverflow) => { }
            Err(e) => { println!("{:?}", e); break;}
        }
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
    }

    final_result
}
