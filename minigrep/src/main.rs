use minigrep::{self, Config};
use std::{env, process};

fn main() {
    let args = env::args().into_iter();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    match minigrep::run(config) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("application error: {e}");
            process::exit(1);
        }
    }
}
