use formally_cool::regular_languages::RegularGrammar;
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::io::Write;

pub struct Rg {}

impl Rg {
    pub fn new() -> Self {
        Rg {}
    }

    fn greatings() {
        println!("{}", "Welcome to the RG tool.");
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
        println!("{:<width$}{}", "exit", "Quit RG tool.", width = width);
        println!(
            "{:<width$}{}",
            "(v)variables",
            "Print variables.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(v)variables add <S> <A> ...",
            "Add space separeted list of '<>' enclosed variables.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(v)variables rm <S> <A> ...",
            "Remove space separeted list of '<>' enclosed variables.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(t)terminals",
            "Print terminals.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(t)terminals add a b ...",
            "Add space separeted list of terminals.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(t)terminals rm a b ...",
            "Remove space separeted list of terminals.",
            width = width
        );
        println!("{:<width$}{}", "(r)rules", "Print rules.", width = width);
        println!(
            "{:<width$}{}",
            "(sv)start_variable",
            "Print start variable.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(sv)start_variable set q0",
            "Set start variable.",
            width = width
        );
    }

    fn wait_for_input() -> Result<String, std::io::Error> {
        print!("{}", "rg> ");

        io::stdout().flush().expect("Error flushing stdout");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Ok(input.trim().to_string()),
            Err(error) => Err(error),
        }
    }

    fn parse_input(input: &str, rg: &mut RegularGrammar) -> Result<(), ()> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.len() != 0 {
            match tokens[0] {
                "help" => Rg::help(),
                "exit" => return Err(()),
                "variables" | "v" => match tokens.iter().nth(1) {
                    Some(&"add") => {}
                    Some(&"rm") => {}
                    Some(other) => println!("unknown command: {}", other),
                    None => println!("{:?}", rg.variables),
                },
                "terminals" | "t" => match tokens.iter().nth(1) {
                    Some(&"add") => {
                        for token in &tokens[2..tokens.len()] {
                            rg.terminals.insert(token.to_string());
                        }
                        println!("{:?}", rg.terminals);
                    }
                    Some(&"rm") => {
                        for token in &tokens[2..tokens.len()] {
                            rg.terminals.remove(&token.to_string());
                        }
                        println!("{:?}", rg.terminals);
                    }
                    Some(other) => println!("unknown command: {}", other),
                    None => println!("{:?}", rg.variables),
                },
                "start_variable" | "sv" => match tokens.iter().nth(1) {
                    Some(&"add") => {}
                    Some(&"rm") => {}
                    Some(other) => println!("unknown command: {}", other),
                    None => println!("{:?}", rg.variables),
                },
                x => {
                    println!("unknown command: {}", x);
                }
            }
        }

        Ok(())
    }

    pub fn run(rg: &mut RegularGrammar) {
        Rg::greatings();

        loop {
            match Rg::wait_for_input() {
                Ok(input) => match Rg::parse_input(&input, rg) {
                    Ok(_) => (),
                    Err(_) => return,
                },
                Err(error) => println!("{}", error),
            };
        }
    }

    pub fn new_rg() -> RegularGrammar {
        RegularGrammar {
            variables: BTreeSet::new(),
            terminals: BTreeSet::new(),
            rules: BTreeMap::new(),
            start_variable: String::new(),
        }
    }
}
