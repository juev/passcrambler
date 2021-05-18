use crate::cmd::parse_args;

extern crate base64;

use rpassword::prompt_password_stdout;
use std::path::Path;
use std::fs;
use std::str;

use crypto::aes::{cbc_encryptor, KeySize};
use crypto::blockmodes::{PkcsPadding};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};

use crypto::digest::Digest;
use crypto::sha3::Sha3;

use base64::{encode};

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

    let data = fs::read_to_string(&args.file)
        .expect("Something went wrong reading the file");

    let password = prompt_password_stdout("Type password: ").unwrap();

    let mut hasher = Sha3::sha3_512();
    // get password digest
    hasher.input(password.as_bytes());
    let pass_digest = hasher.result_str();
    hasher.reset();

    // get login digest
    hasher.input((&args.login).as_bytes());
    let login_digest = hasher.result_str();
    hasher.reset();

    // print login and password digest
    println!("login digest: {}", login_digest);
    println!("password digest: {}", pass_digest);

    // find aes coding our data
    let aes_out1 = aes_encrypt(
        &*data.into_bytes(),
        pass_digest.as_bytes(),
        login_digest.as_bytes(),
    );

    hasher.input(&aes_out1);
    let sha_digest = hasher.result_str();
    hasher.reset();
    println!("aes1 digest: {}", sha_digest);

    let passlen = password.len() % sha_digest.len();
    let key2 = &sha_digest[passlen..passlen+32];
    println!("passlen: {}", passlen);
    println!("key2: {}", key2);

    // aes_out2 = aes_encrypt( key, key2, aes_out1 )
    // find aes coding our data
    let aes_out2 = aes_encrypt(
        &*aes_out1,
        pass_digest.as_bytes(),
        key2.as_bytes(),
    );

    // start    = key[0] % len(aes_out2)
    let start = password.chars().nth(0).unwrap() as usize % aes_out2.len();
    // portion  = aes_out2[start:]
    let portion = &aes_out2[start..aes_out2.len()];
    // result   = hashlib.sha512(portion).digest()
    hasher.input(&portion);
    let result = hasher.result_str();
    hasher.reset();
    // longpass = base64.b64encode(result)
    // longpass = longpass[0:args.length]
    let base = encode(&result);
    let longpass = str::from_utf8(&base.as_bytes()[..args.length as usize]).unwrap();
    println!("longpass: {:}", longpass);
    println!("longpass length: {}", longpass.len());
    // longpass = convert_to_charset(longpass,  sorted(args.special, reverse=True))

}

pub fn aes_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut encryptor = cbc_encryptor(
        KeySize::KeySize256,
        &key[..16],
        &iv[..16],
        PkcsPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut buffer = [0; 16];
    let mut read_buffer = RefReadBuffer::new(data);
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    loop {
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
