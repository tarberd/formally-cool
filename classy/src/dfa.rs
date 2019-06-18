use formally_cool::regular_languages::DeterministicFiniteAutomata;
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::io::Write;

pub struct Dfa {}

impl Dfa {
    pub fn new() -> Self {
        Dfa {}
    }

    fn greatings() {
        println!("{}", "Welcome to the DFA tool.");
        println!("{}", "Type 'help' for available commands.");
    }

    fn help() {
        println!("{}", "List of available commands:");
        println!("{:<25}{}", "help", "Show available commands.");
        println!("{:<25}{}", "exit", "Quit DFA tool.");
        println!("{:<25}{}", "states", "List states.");
        println!(
            "{:<25}{}",
            "states add q0 q1 ...", "Add space separeted list of states."
        );
        println!(
            "{:<25}{}",
            "states rm q0 q1 ...", "Remove space separeted list of states."
        );
        println!("{:<25}{}", "alphabet", "List alphabet.");
        println!(
            "{:<25}{}",
            "alphabet add a b ...", "Add space separeted list of letters."
        );
        println!(
            "{:<25}{}",
            "alphabet rm a b ...", "Remove space separeted list of letters."
        );
    }

    fn wait_for_input() -> Result<String, std::io::Error> {
        print!("{}", "dfa> ");

        io::stdout().flush().expect("Error flushing stdout");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Ok(input.trim().to_string()),
            Err(error) => Err(error),
        }
    }

    fn parse_input(input: &str, dfa: &mut DeterministicFiniteAutomata) -> Result<(), ()> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.len() != 0 {
            match tokens[0] {
                "help" => Dfa::help(),
                "exit" => return Err(()),
                "states" => match tokens.iter().nth(1) {
                    Some(operation) => match operation {
                        &"add" => {}
                        &"rm" => {}
                        x => println!("{} is not a valid operation.", *x),
                    },
                    None => println!("{:?}", dfa.states),
                },
                "alphabet" => match tokens.iter().nth(1) {
                    Some(operation) => match operation {
                        &"add" => {
                            for token in &tokens[2..tokens.len()] {
                                dfa.alphabet.insert(token.to_string());
                            }
                            println!("{:?}", dfa.alphabet);
                        }
                        &"rm" => {
                            for token in &tokens[2..tokens.len()] {
                                dfa.alphabet.remove(&token.to_string());
                            }
                            println!("{:?}", dfa.alphabet);
                        }
                        x => println!("{} is not a valid operation.", *x),
                    },
                    None => println!("{:?}", dfa.alphabet),
                },
                x => {
                    println!("unknown command: {}", x);
                }
            }
        }

        Ok(())
    }

    pub fn run(dfa: &mut DeterministicFiniteAutomata) {
        Dfa::greatings();

        loop {
            match Dfa::wait_for_input() {
                Ok(input) => match Dfa::parse_input(&input, dfa) {
                    Ok(_) => (),
                    Err(_) => return,
                },
                Err(error) => println!("{}", error),
            };
        }
    }

    pub fn new_dfa() -> DeterministicFiniteAutomata {
        DeterministicFiniteAutomata {
            states: BTreeSet::new(),
            alphabet: BTreeSet::new(),
            transition_function: BTreeMap::new(),
            start_state: String::new(),
            accept_states: BTreeSet::new(),
        }
    }
}
