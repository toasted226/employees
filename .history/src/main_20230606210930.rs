use std::{io::{self, Write}, collections::HashMap};

struct KeywordActive {
    name: String,
}

fn main() {
    let input = prompt_string("Enter a prompt: ");

    let mut names: Vec<String> = Vec::new();
    
    for word in input.split_whitespace() {
        match word {
            "add" => add = true,
            "to" => {
                add = false;
                to = true;
            }
            _ => {

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
