use std::io;
use std::io::Write;

mod command;
use command::*;

struct Classy {}

impl Classy {
    fn new() -> Self {
        Classy {}
    }

    fn greatings() {
        println!("{}", "Welcome to the classy cli tool.");
        println!("{}", "Type \"help\" for available commands.");
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
    }

    fn run(&mut self) {
        Classy::greatings();

        match Classy::wait_for_input() {
            Ok(input) => self.parse_input(&input),
            Err(error) => println!("{}", error),
        };
    }
}

fn main() {
    let mut app = Classy::new();

    app.run();
}
