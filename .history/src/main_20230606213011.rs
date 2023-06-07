use std::{io::{self, Write}, collections::HashMap};

enum KeywordActive {
    Add,
    To,
}

fn main() {
    let input = prompt_string("Enter a prompt: ");

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    let mut names: Vec<String> = Vec::new();
    let mut keyword: KeywordActive = KeywordActive::Add;

    for word in input.split_whitespace() {
        match word {
            "add" => keyword = KeywordActive::Add,
            "to" => keyword = KeywordActive::To,
            _ => {
                match keyword {
                    KeywordActive::Add => names.push(word.to_string()),
                    KeywordActive::To => add_employees_to_department(&names, word, &mut departments),
                }
            }
        }
    }
}

fn prompt_string(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush output stream!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    input
}

fn add_employees_to_department(names: &Vec<String>, department: &str, departments_map: &mut HashMap<String, Vec<String>>) {
    for name in names {
        let v: Vec<String> = Vec::new();
        let department_names = departments_map.entry(department.to_string()).or_insert(v);
        department_names.push(name.to_string());
    }
}
