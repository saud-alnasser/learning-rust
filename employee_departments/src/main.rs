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

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let option = loop {
            match prompt("what do you want to do? (add/view/exit): ") {
                Ok(option) => match option.as_str() {
                    "add" | "view" | "exit" => break option,
                    _ => {
                        println!("provided option is not valid, please type either 'add' or 'view'")
                    }
                },
                Err(_) => println!("unexpected error occurred, please try again"),
            }
        };

        let departments_list = departments
            .keys()
            .map(|k| k.as_str())
            .collect::<Vec<&str>>()
            .join(",");

        match option.as_str() {
            "add" => {
                let name = loop {
                    match prompt("what is the employee's name? ") {
                        Ok(name) => break name,
                        Err(_) => println!("unexpected error occurred, please try again"),
                    }
                };

                let department = loop {
                    match prompt(
                        format!(
                            "departments: [{}]\nwhat department does the employee belong to? ",
                            departments_list
                        )
                        .as_str(),
                    ) {
                        Ok(department) => break department,
                        Err(_) => println!("unexpected error occurred, please try again"),
                    }
                };

                departments.entry(department).or_insert(vec![name]);
            }
            "view" => {
                let department = loop {
                    match prompt(
                        format!(
                            "departments: [{}]\nwhat department do you want to view? ",
                            departments_list
                        )
                        .as_str(),
                    ) {
                        Ok(department) => match departments_list.contains(&department) {
                            true => break department,
                            false => println!("provided department is not valid"),
                        },
                        Err(_) => println!("unexpected error occurred, please try again"),
                    }
                };

                let employees = departments.entry(department.clone()).or_default();

                println!(
                    "employees of {} department: [{}]",
                    department,
                    employees.join(", ")
                );
            }
            "exit" => break,
            _ => unreachable!(),
        }
    }
}
