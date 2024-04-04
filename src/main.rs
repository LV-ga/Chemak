use std::io::{self, Write};
mod atoms;
mod bond_cycle;
mod elements;
mod input_handler;

fn main() {
    print!("Enter some text (only specific format allowed): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.retain(|c| !c.is_whitespace());

    match input_handler::extract_substrings(&input) {
        Ok(substrings) => {
            println!("Valid substrings: {:?}", substrings);
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}
