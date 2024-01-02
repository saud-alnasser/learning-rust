use std::{
    cmp::Ordering,
    io::{self, Write},
};

use rand::Rng;

fn main() {
    println!("guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("enter your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read-line!");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("guess must be a number!");

                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you won!");

                break;
            }
        }
    }
}
