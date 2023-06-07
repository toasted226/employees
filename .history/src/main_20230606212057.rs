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
                    KeywordActive::To => add_employees_to_department(&names, word, &departments),
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

fn add_employees_to_department(names: &Vec<String>, department: &str, departmentsMap: &HashMap<String, Vec<String>>) {

}