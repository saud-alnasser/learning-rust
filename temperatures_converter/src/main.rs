use std::io::{self, Write};

fn main() {
    let from_temperature: f64 = loop {
        match prompt("enter temperature: ") {
            Ok(input) => match input.parse() {
                Ok(num) => break num,
                Err(_) => println!("temperature must be a number!"),
            },
            Err(_) => println!("failed to read line"),
        };
    };

    let from_unit: char = loop {
        match prompt("enter unit (C|F): ") {
            Ok(input) => match input.to_uppercase().as_str() {
                "C" => break 'C',
                "F" => break 'F',
                _ => println!("unit must be \"C\" or \"F\"!"),
            },
            Err(_) => println!("failed to read line"),
        };
    };

    let to_temperature: f64 = match from_unit {
        'C' => from_temperature * 9.0 / 5.0 + 32.0,
        'F' => (from_temperature - 32.0) * 5.0 / 9.0,
        _ => panic!("unreachable state!!!"),
    };

    let to_unit: char = match from_unit {
        'C' => 'F',
        'F' => 'C',
        _ => panic!("unreachable state!!!"),
    };

    println!("from {from_temperature:.2}{from_unit} to {to_temperature:.2}{to_unit}",);
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
