use std::{io::{self, Write}, collections::HashMap};

enum Keyword {
    Invalid,
    Add,
    To,
    List,
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let input = prompt_string("Enter a prompt: ");
        if input.trim() == "quit" {
            break;
        }
        process_command(&input, &mut departments);

        println!("{:?}", departments);
    }
}

fn prompt_string(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush output stream!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    input
}

fn process_command(input: &String, map: &mut HashMap<String, Vec<String>>) {
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
    let mut departments: Vec<String> = Vec::new();

    for (department, employees) in map {
        departments.push(department);
    }
}

fn list_department_members(department: &str, map: &mut HashMap<String, Vec<String>>) {

}