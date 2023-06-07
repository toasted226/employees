use std::{io::{self, Write}, collections::HashMap};

fn main() {
    let input = prompt_string("Enter a prompt: ");
}

fn prompt_string(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush output stream!");
    let mut input = String::new();
}
