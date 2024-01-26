use std::{
    collections::HashMap,
    io::{self, Write},
};

fn prompt(prompt: &str) -> Result<String, io::Error> {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => Err(e),
    }
}

fn mean(numbers: &[i32]) -> Option<f64> {
    let sum = numbers.iter().sum::<i32>();
    let length = numbers.len();

    match length == 0 {
        true => None,
        false => Some(sum as f64 / length as f64),
    }
}

fn median(numbers: &[i32]) -> Option<f64> {
    if numbers.len() == 0 {
        return None;
    }

    let mut numbers = numbers.to_vec();

    numbers.sort();

    let mid = numbers.len() / 2;

    match numbers.len() % 2 == 0 {
        true => Some((numbers[mid - 1] + numbers[mid]) as f64 / 2.0),
        false => Some(numbers[mid] as f64),
    }
}

fn mode(numbers: &[i32]) -> Option<i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for num in numbers {
        let count = counts.entry(*num).or_insert(0);
        *count += 1;
    }

    counts.into_iter().max_by_key(|x| x.1).map(|x| x.0)
}

fn main() {
    let input = prompt("enter integers i.e. (1,2,3,4,5): ").unwrap();

    let list = input
        .split(",")
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("mean: {}", mean(&list).unwrap_or(0.0));
    println!("median: {}", median(&list).unwrap_or(0.0));
    println!("mode: {}", mode(&list).unwrap_or(0));
}
