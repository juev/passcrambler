const HELP: &str = concat!(
    env!("CARGO_PKG_NAME"),
    ": v.",
    env!("CARGO_PKG_VERSION"),
    "\n\n",
    "USAGE:
  passcrambler [OPTIONS]

FLAGS:
  -h, --help            Prints help information
  -c, --clip            Copy the generated password into the clipboard instead of displaying

OPTIONS:
  -f, --file FILE       File for seeding password, REQUIRED
  -l, --login LOGIN     Login data for password, REQUIRED
  -L, --length 30       Length of the password, default=30
  -s, --symbols '_&#'   Symbols for using in password, default='_-&#*^%$@!~'
"
);

#[derive(Debug)]
pub struct AppArgs {
    pub file: String,
    pub login: String,
    pub length: u32,
    pub clip: bool,
    pub symbols: String,
}

pub fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let file: String = pargs.value_from_str(["-f", "--file"])?;
    let login: String = pargs.value_from_str(["-l", "--login"])?;
    let length: u32 = pargs
        .opt_value_from_fn(["-L", "--length"], parse_length)?
        .unwrap_or(30);
    let symbols: String = pargs
        .value_from_str(["-s", "--symbols"])
        .unwrap_or("_-&#*^%$@!~".to_string());

    Ok(AppArgs {
        file,
        login,
        length,
        clip: pargs.contains(["-c", "--clip"]),
        symbols,
    })
}

fn parse_length(s: &str) -> Result<u32, &'static str> {
    s.parse().map_err(|_| "not a number")
}
