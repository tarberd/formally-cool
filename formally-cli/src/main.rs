use formally_cool::regular_languages::*;
use std::io;

fn greet() {
    println!("formally-cli interface:");
    println!("Type \"help\" for available commands.");
}

fn help() {
    println!("Available commands:");
    println!("exit");
    println!("dfa_to_rg");
}

fn dfa_to_rg() {
    println!("Input transition function: (format:'state' 'symbol' 'next_state')");

    let mut input = String::new();
}

fn main() {
    greet();

    let mut input = String::new();

    while input.trim() != "exit" {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "help" {
                    help();
                }
                if input.trim() == "dfa_to_rg" {
                    dfa_to_rg();
                }
                input = String::new();
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
