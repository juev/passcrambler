extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Password scrambler")
        .version("0.1.0")
        .author("Denis Evsyukov <denis@evsyukov.org>")
        .arg(
            Arg::with_name("file")
                .long("file")
                .takes_value(true)
                .required(true)
                .value_name("FILE")
                .help("File used to initialize generation"),
        )
        .arg(
            Arg::with_name("login")
                .long("login")
                .takes_value(true)
                .required(true)
                .help("Login for which you want to use the password"),
        )
        .arg(
            Arg::with_name("special")
                .long("special")
                .takes_value(true)
                .default_value("_&#")
                .help("Whitelist of special characters, i.e: '_&#'"),
        )
        .arg(
            Arg::with_name("length")
                .long("length")
                .takes_value(true)
                .default_value("30")
                .help("Length of the password, default=30"),
        )
        .arg(
            Arg::with_name("clip")
                .long("clip")
                .help("Copy the generated password into the clipboard instead of displaying"),
        )
        .arg(
            Arg::with_name("func")
                .long("scramble-func")
                .takes_value(true)
                .default_value("md5")
                .help("Hashing function to use for input data scrambling, default=md5."),
        )
        .get_matches();
    let file = matches.value_of("file").unwrap();
    let login = matches.value_of("login").unwrap();
    let special = matches.value_of("special").unwrap();
    println!("{}\n{}\n{}", file, login, special);
}
