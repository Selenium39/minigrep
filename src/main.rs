use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        process::exit(1);
    });
    if let Err(err) = minigrep::run(config) {
        process::exit(1);
    }
}
