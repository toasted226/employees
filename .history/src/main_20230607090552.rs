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
    let mut names: Vec<String> = Vec::new();
    let mut keyword: Keyword = Keyword::Invalid;
    
    for word in input.split_whitespace() {
        match word {
            "add" => keyword = Keyword::Add,
            "to" => keyword = Keyword::To,
            "list" => keyword = Keyword::List,
            _ => {
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

fn add_employees(names: &Vec<String>, department: &str, map: &mut HashMap<String, Vec<String>>) {
    for name in names {
        let v: Vec<String> = Vec::new();
        let department_names = map.entry(department.to_string()).or_insert(v);
        department_names.push(name.to_string());
    }
}

fn list(list_by: &str, map: &mut HashMap<String, Vec<String>>) {
    match list_by {
        "all" => list_all(map),
        _ => list_department_members(list_by, map),
    }
}

fn list_all(map: &mut HashMap<String, Vec<String>>) {
    for (department, employees) in map {
        println!("{department}: ");
        employees.sort();
        for employee in employees {
            println!(" - {employee}");
        }
        println!("");
    }
}

fn list_department_members(department: &str, map: &mut HashMap<String, Vec<String>>) {
    if map.contains_key(department) {
        println!("{department}: ");
        if let Some(employees) = map.get_mut(department) {
            employees.sort();
            for employee in employees {
                println!(" - {employee}");
            }
        }
    }
}
