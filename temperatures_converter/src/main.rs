use std::io::{self, Write};

fn main() {
    let (from_temperature, from_unit): (f64, char) = loop {
        match prompt("enter temperature: ") {
            Ok(input) => match (input[0..input.len() - 1].parse(), input.chars().last()) {
                (Ok(temperature), Some('C')) => break (temperature, 'C'),
                (Ok(temperature), Some('F')) => break (temperature, 'F'),
                _ => println!("temperature must in this format (e.g. 37C or 98.6F)"),
            },
            Err(_) => println!("failed to read-line!"),
        };
    };

    let (to_temperature, to_unit): (f64, char) = match from_unit {
        'C' => (from_temperature * 9.0 / 5.0 + 32.0, 'F'),
        'F' => ((from_temperature - 32.0) * 5.0 / 9.0, 'C'),
        _ => panic!("unreachable state!!!"),
    };

    println!("converted temperature: {to_temperature:.2}{to_unit}",);
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
