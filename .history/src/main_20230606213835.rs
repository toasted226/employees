use std::{io::{self, Write}, collections::HashMap};

enum KeywordActive {
    Add,
    To,
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    let input = prompt_string("Enter a prompt: ");

    println!("{:?}", departments);
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
    let mut keyword: KeywordActive = KeywordActive::Add;
    
    for word in input.split_whitespace() {
        match word {
            "add" => keyword = KeywordActive::Add,
            "to" => keyword = KeywordActive::To,
            _ => {
                match keyword {
                    KeywordActive::Add => names.push(word.to_string()),
                    KeywordActive::To => add_employees(&names, word, map),
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
