const HELP: &str = concat!(
    env!("CARGO_PKG_NAME"),
    ": v.",
    env!("CARGO_PKG_VERSION"),
    "\n\n",
    "USAGE:
  app [OPTIONS] --file FILE --login LOGIN

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --special STR         Whitelist of special characters, i.e: '_&#'
  --opt-number NUMBER   Sets an optional number
  --length 30           Length of the password, default=30
  -c, --clip            Copy the generated password into the clipboard instead of displaying
  --scramble-func md5   Hashing function to use for input data scrambling, default=md5.
"
);

#[derive(Debug)]
pub struct AppArgs {
    pub file: String,
    pub login: String,
    pub special: String,
    pub length: u32,
    pub clip: bool,
    pub func: String,
}

pub fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let file: String = pargs.value_from_str("--file")?;
    let login: String = pargs.value_from_str("--login")?;
    let special: String = pargs
        .value_from_str("--special")
        .unwrap_or("_&#".to_string());
    let length: u32 = pargs
        .opt_value_from_fn("--length", parse_length)?
        .unwrap_or(30);
    let func: String = pargs
        .value_from_str("--scramble-func")
        .unwrap_or("md5".to_string());

    Ok(AppArgs {
        file,
        login,
        special,
        length,
        clip: pargs.contains(["-c", "--clip"]),
        func,
    })
}

fn parse_length(s: &str) -> Result<u32, &'static str> {
    s.parse().map_err(|_| "not a number")
}
