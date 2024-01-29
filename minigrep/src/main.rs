use minigrep::{self, Config};
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("parsing arguments error: {e}");
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
