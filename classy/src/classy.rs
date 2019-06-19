use crate::dfa::Dfa;
use formally_cool::regular_languages::DeterministicFiniteAutomata;
use std::collections::HashMap;
use std::io;
use std::io::Write;

pub struct Classy {
    id_to_dfa: HashMap<String, DeterministicFiniteAutomata>,
}

impl Classy {
    pub fn new() -> Self {
        Classy {
            id_to_dfa: HashMap::new(),
        }
    }

    fn greatings() {
        println!("{}", "Welcome to the classy cli tool.");
        println!("{}", "Type 'help' for available commands.");
    }

    fn help() {
        let width = 40;
        println!("{}", "List of available commands:");
        println!(
            "{:<width$}{}",
            "help",
            "Show available commands.",
            width = width
        );
        println!("{:<width$}{}", "exit", "Quit classy.", width = width);
        println!(
            "{:<width$}{}",
            "let [id] = [expression]",
            "Store on 'id' value returned by 'expression'.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "[expression] => new dfa",
            "Create empty DFA.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "[expression] => read dfa [file_name]",
            "Create new object from file.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "write [id] [file_name]",
            "Write object to file.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "edit [id]",
            "Open edit context for [id] object.",
            width = width
        );
    }

    fn wait_for_input() -> Result<String, std::io::Error> {
        print!("{}", ">>> ");

        io::stdout().flush().expect("Error flushing stdout.");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Ok(input.trim().to_string()),
            Err(error) => Err(error),
        }
    }

    fn parse_input(&mut self, input: &str) {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.len() != 0 {
            match tokens[0] {
                "help" => Classy::help(),
                "exit" => std::process::exit(0),
                "let" => {
                    match tokens.iter().nth(1) {
                        Some(id) => match tokens.iter().nth(2) {
                            Some(assingment) => {
                                if *assingment == "=" {
                                    match tokens.iter().nth(3) {
                                        Some(&"new") => match tokens.iter().nth(4) {
                                            Some(x) => {
                                                if *x == "dfa" {
                                                    let mut dfa = Dfa::new_dfa();
                                                    Dfa::run(&mut dfa);
                                                    println!("{}", dfa);
                                                    self.id_to_dfa.insert(id.to_string(), dfa);
                                                } else {
                                                    println!("unknown type: {}.", *x);
                                                }
                                            }
                                            None => {
                                                println!("Expected type after new for {}.", *id)
                                            }
                                        },
                                        Some(&"read") => match tokens.iter().nth(4) {
                                            Some(x) => {
                                                if *x == "dfa" {
                                                    match tokens.iter().nth(5) {
                                                        Some(file_name) => {
                                                            match std::fs::File::open(file_name) {
                                                                Ok(file) => {
                                                                    let reader =
                                                                        std::io::BufReader::new(
                                                                            file,
                                                                        );
                                                                    match serde_yaml::from_reader(reader) {
                                                                            Ok(dfa) => {
                                                                                self.id_to_dfa
                                                                                    .insert(id.to_string(), dfa);
                                                                            }
                                                                            Err(err) => println!(
                                                                                "Error parsing file {}: {}",
                                                                                file_name, err
                                                                            ),
                                                                    };
                                                                }
                                                                Err(err) => println!(
                                                                    "Error opening file {}: {}",
                                                                    file_name, err
                                                                ),
                                                            }
                                                        }
                                                        None => println!(
                                                            "Expected file_name after read for {}.",
                                                            *id
                                                        ),
                                                    }
                                                } else {
                                                    println!("unknown type: {}", *x);
                                                }
                                            }
                                            None => {
                                                println!("Expected type after new for {}.", *id)
                                            }
                                        },
                                        Some(x) => println!("unknown operator: {}", x),
                                        None => println!("Expected expression after assingment."),
                                    }
                                } else {
                                    println!("Expected = after id, found {}", assingment);
                                }
                            }
                            None => println!("Expected = after id."),
                        },
                        None => println!("Expected id after let expression."),
                    };
                }
                "write" => match tokens.iter().nth(1) {
                    Some(id) => {
                        if self.id_to_dfa.contains_key(&id.to_string()) {
                            match tokens.iter().nth(2) {
                                Some(file_name) => match self.id_to_dfa.get(&id.to_string()) {
                                    Some(dfa) => match std::fs::File::create(file_name) {
                                        Ok(file) => {
                                            let writer = std::io::BufWriter::new(file);
                                            match serde_yaml::to_writer(writer, &dfa) {
                                                Ok(_) => (),
                                                Err(err) => println!(
                                                    "Error writing {} to {}: {}",
                                                    id, file_name, err
                                                ),
                                            }
                                        }
                                        Err(e) => println!("error : {:?}", e),
                                    },
                                    None => (),
                                },
                                None => println!("Expected file_name after write for {}.", *id),
                            }
                        } else {
                            println!("unknown id: {}", id);
                        }
                    }
                    None => println!("Expected id after edit."),
                },
                "edit" => match tokens.iter().nth(1) {
                    Some(id) => {
                        if self.id_to_dfa.contains_key(&id.to_string()) {
                            match self.id_to_dfa.get_mut(&id.to_string()) {
                                Some(mut dfa) => {
                                    Dfa::run(&mut dfa);
                                    println!("{}", dfa);
                                }
                                None => (),
                            }
                        } else {
                            println!("unknown id: {}", id);
                        }
                    }
                    None => println!("Expected id after edit."),
                },
                x => {
                    if self.id_to_dfa.contains_key(&x.to_string()) {
                        match self.id_to_dfa.get(&x.to_string()) {
                            Some(dfa) => println!("{}", dfa),
                            None => (),
                        }
                    } else {
                        println!("unknown command: {}", x);
                    }
                }
            }
        }
    }

    pub fn run(&mut self) {
        Classy::greatings();

        loop {
            match Classy::wait_for_input() {
                Ok(input) => self.parse_input(&input),
                Err(error) => println!("{}", error),
            };
        }
    }
}
