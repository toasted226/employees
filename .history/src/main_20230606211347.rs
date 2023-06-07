use std::{io::{self, Write}, collections::HashMap};

enum KeywordActive {
    Add,
    To,
}

fn main() {
    let input = prompt_string("Enter a prompt: ");

    let mut names: Vec<String> = Vec::new();
    let mut keyword: KeywordActive = KeywordActive::Add;
    for word in input.split_whitespace() {
        match word {
            "add" => keyword = KeywordActive::Add,
            "to" => keyword = KeywordActive::To,
            _ => {
                match keyword {
                    KeywordActive::Add => names.push(word.to_string())
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
