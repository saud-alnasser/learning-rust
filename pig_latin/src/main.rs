use std::io::{self, Write};

fn prompt(prompt: &str) -> Result<String, io::Error> {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => Err(e),
    }
}

fn is_vowel(c: &char) -> bool {
    "aeiou".contains(*c)
}

fn to_pig_latin(word: &String) -> String {
    let lowered = word.to_lowercase();
    let mut chars = lowered.chars();

    let first_letter = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    let rest_of_letters = chars.as_str();

    match is_vowel(&first_letter) {
        true => format!("{}{}-hay", first_letter, rest_of_letters),
        false => format!("{}-{}ay", rest_of_letters, first_letter),
    }
}

fn main() {
    let word = prompt("word: ").unwrap();
    let converted = to_pig_latin(&word);

    println!("converted: {}", converted)
}
