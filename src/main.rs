use crate::cmd::parse_args;

use rpassword::prompt_password_stdout;
use std::fs;
use std::path::Path;

use rand::prelude::*;
use rand_pcg::Pcg64;

use crypto::aes::{cbc_encryptor, KeySize};
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};

use crypto::digest::Digest;
use crypto::sha3::Sha3;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use anyhow::Error;
use fehler::throws;

pub mod cmd;

#[throws]
fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    if !Path::new(&args.file).exists() {
        println!("file not exists");
        std::process::exit(1);
    }

    let data = fs::read_to_string(&args.file).expect("Something went wrong reading the file");

    let password = prompt_password_stdout("Type password: ")?;

    let mut hasher = Sha3::sha3_512();
    // get password digest
    hasher.input(password.as_bytes());
    let pass_digest = hasher.result_str();
    hasher.reset();

    // get login digest
    hasher.input((&args.login).as_bytes());
    let login_digest = hasher.result_str();
    hasher.reset();

    // find aes coding our data
    let aes_out1 = aes_encrypt(
        &*data.into_bytes(),
        pass_digest.as_bytes(),
        login_digest.as_bytes(),
    );

    hasher.input(&aes_out1);
    let sha_digest = hasher.result_str();
    hasher.reset();

    let mut rng: Pcg64 = rand_seeder::Seeder::from(sha_digest).make_rng();

    let mut symbols = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string();
    symbols.push_str(&args.symbols);

    let long_password: String = (0..args.length)
        .map(|_| {
            let idx = rng.gen_range(0..symbols.len());
            (symbols.as_bytes())[idx] as char
        })
        .collect();

    if args.clip {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(long_password).unwrap();
    } else {
        println!("---\n{}", long_password);
    }
}

pub fn aes_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut encryptor = cbc_encryptor(KeySize::KeySize256, &key[..16], &iv[..16], PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut buffer = [0; 16];
    let mut read_buffer = RefReadBuffer::new(data);
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);
        match result {
            Ok(BufferResult::BufferUnderflow) => break,
            Ok(BufferResult::BufferOverflow) => {}
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .copied(),
        );
    }

    final_result
}
