use std::{io::{self, Write}, collections::HashMap};

enum Keyword {
    Invalid,
    Add,
    To,
    List,
}

fn main() {
    // Create hashmap to store departments and employees for each
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    // Prompt the user repeatedly until they quit
    loop {
        let input = prompt_string("Enter a prompt: ");
        if input.trim() == "quit" {
            break;
        }
        // Process their query
        process_query(&input, &mut departments);
        // println!("{:?}", departments);
    }
}

// Prompt the user to enter text and return the String
fn prompt_string(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush output stream!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    input
}

// Process the user's query
fn process_query(input: &String, map: &mut HashMap<String, Vec<String>>) {
    // Store the current keyword used and the names user listed
    let mut names: Vec<String> = Vec::new();
    let mut keyword: Keyword = Keyword::Invalid;
    
    // Process each word in query
    for word in input.split_whitespace() {
        match word {
            // Change current keyword used
            "add" => keyword = Keyword::Add,
            "to" => keyword = Keyword::To,
            "list" => keyword = Keyword::List,
            _ => {
                // Do something with the word based on the active keyword
                match keyword {
                    Keyword::Add => names.push(word.to_string()),
                    Keyword::To => add_employees(&names, word, map),
                    Keyword::List => list(word, map),
                    _ => println!("Unrecognised keyword '{word}'"),
                }
            }
        }
    }
}

// Add employees to a specific department
fn add_employees(names: &Vec<String>, department: &str, map: &mut HashMap<String, Vec<String>>) {
    for name in names {
        let v: Vec<String> = Vec::new();
        let department_names = map.entry(department.to_string()).or_insert(v);
        department_names.push(name.to_string());
    }
}

// Call either list all or list a specific department
fn list(list_by: &str, map: &mut HashMap<String, Vec<String>>) {
    match list_by {
        "all" => list_all(map),
        _ => list_department_members(list_by, map),
    }
}

// List all departments and their employees
fn list_all(map: &mut HashMap<String, Vec<String>>) {
    for (department, employees) in map {
        println!("{department}: ");
        // Sort employees alphabetically
        employees.sort();
        for employee in employees {
            println!(" - {employee}");
        }
        println!("");
    }
}

// List all employees in a specific department
fn list_department_members(department: &str, map: &mut HashMap<String, Vec<String>>) {
    // if department exists
    if map.contains_key(department) {
        println!("{department}: ");
        // Get employees vec, sort them alphabetically and display them
        if let Some(employees) = map.get_mut(department) {
            employees.sort();
            for employee in employees {
                println!(" - {employee}");
            }
        }
    }
}
