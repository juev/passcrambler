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
  -L, --length 30       Length of the password, default=30
  -c, --clip            Copy the generated password into the clipboard instead of displaying
"
);

#[derive(Debug)]
pub struct AppArgs {
    pub file: String,
    pub login: String,
    pub length: u32,
    pub clip: bool,
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

    Ok(AppArgs {
        file,
        login,
        length,
        clip: pargs.contains(["-c", "--clip"]),
    })
}

fn parse_length(s: &str) -> Result<u32, &'static str> {
    s.parse().map_err(|_| "not a number")
}
