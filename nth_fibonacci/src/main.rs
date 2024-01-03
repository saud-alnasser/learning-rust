use std::io::{self, Write};

fn fibonacci(n: u32) -> u64 {
    if n == 0 || n == 1 {
        return n as u64;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn prompt(prompt: &str) -> Result<String, io::Error> {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => Err(e),
    }
}

fn main() {
    let n: u32 = loop {
        match prompt("enter nth fibonacci: ") {
            Ok(input) => match input.parse() {
                Ok(n) => match n > 50 {
                    true => println!("n cannot exceed 50!"),
                    false => break n,
                },
                Err(_) => println!("n must be a number!"),
            },
            Err(_) => println!("failed to read-line!"),
        }
    };

    println!("{}th fibonacci is {}", n, fibonacci(n));
}
