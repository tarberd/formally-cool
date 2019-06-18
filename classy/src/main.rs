use std::io;
use std::io::Write;

// use formaly_cool::regular_languages::*;

struct Classy {}

impl Classy {
    fn new() -> Self {
        Classy {}
    }

    fn greatings() {
        println!("{}", "Welcome to the classy cli tool.");
        println!("{}", "Type 'help' for available commands.");
    }

    fn help() {
        println!("{}", "List of available commands:");
        println!("{:<25}{}", "help", "Show available commands.");
        println!(
            "{:<25}{}",
            "let [id] = [expression]", "Store on 'id' value returned by 'expression'."
        );
    }

    fn wait_for_input() -> Result<String, std::io::Error> {
        print!("{}", ">>> ");

        io::stdout().flush().expect("Error flushing stdout");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Ok(input.trim().to_string()),
            Err(error) => Err(error),
        }
    }

    fn parse_input(&self, input: &str) {
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
                                                    // TODO let dfa = create_dfa();
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
                x => println!("unknown command: {}", x),
            }
        }
    }

    fn run(&mut self) {
        Classy::greatings();

        loop {
            match Classy::wait_for_input() {
                Ok(input) => self.parse_input(&input),
                Err(error) => println!("{}", error),
            };
        }
    }
}

fn main() {
    let mut app = Classy::new();

    app.run();
}
