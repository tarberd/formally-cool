use formally_cool::regular_languages::NondeterministicFiniteAutomata;
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::io::Write;

pub struct Nfa {}

impl Nfa {
    pub fn new() -> Self {
        Nfa {}
    }

    fn greatings() {
        println!("{}", "Welcome to the NFA tool.");
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
        println!("{:<width$}{}", "exit", "Quit DFA tool.", width = width);
        println!("{:<width$}{}", "(s)states", "Print states.", width = width);
        println!(
            "{:<width$}{}",
            "(s)states add q0 (q1, q2) ...",
            "Add space separeted list of states.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(s)states rm q0 (q1, q2) ...",
            "Remove space separeted list of states.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(a)alphabet",
            "Print alphabet.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(a)alphabet add a b ...",
            "Add space separeted list of letters.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(a)alphabet rm a b ...",
            "Remove space separeted list of letters.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(t)transition",
            "Print transition table.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(t)transition add [state] ; [letter] -> [state] [state] ... | [state] ; [letter] -> [state] [state] ... | ...",
            "\n\tAdd list of transitions separeted by '|'.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(t)transition add [state] ; [letter] -> [state] [state] ... | [state] ; [letter] -> [state] [state] ... | ...",
            "\n\tRemove list of transitions separeted by '|'.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(ss)start_state",
            "Print start state.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(ss)start_state set q0",
            "Set start_state.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(as)accept_states",
            "Print accept states.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(as)accept_states add q0 (q1, q2) ...",
            "Add space separeted list of states.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(as)accept_states rm q0 (q1, q2) ...",
            "Remove space separeted list of states.",
            width = width
        );
    }

    fn wait_for_input() -> Result<String, std::io::Error> {
        print!("{}", "nfa> ");

        io::stdout().flush().expect("Error flushing stdout");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Ok(input.trim().to_string()),
            Err(error) => Err(error),
        }
    }

    fn tokens_to_states(tokens: &[&str]) -> BTreeSet<String> {
        let mut states = BTreeSet::new();

        if tokens.len() != 0 {
            let mut parentheses_count = 0;
            let mut state = String::new();

            for index in 0..tokens.len() {
                let fist_letter = tokens[index].get(0..1).unwrap();
                let end = tokens[index].len();
                let last_letter = tokens[index].get((end - 1)..end).unwrap();

                if parentheses_count == 0 {
                    if fist_letter == "(" {
                        state += tokens[index];

                        if last_letter == ")" {
                            states.insert(state.clone());
                            state = String::new()
                        } else {
                            parentheses_count += 1;
                        }
                    } else {
                        states.insert(tokens[index].to_string());
                    }
                } else {
                    state = state + " " + tokens[index];
                    if last_letter == ")" {
                        parentheses_count -= 1;

                        if parentheses_count == 0 {
                            states.insert(state.clone());
                            state = String::new();
                        }
                    }
                }
            }
        }

        states
    }

    fn tokens_to_transitions(tokens: &[&str]) -> BTreeMap<(String, String), BTreeSet<String>> {
        let mut transition_function = BTreeMap::new();

        if tokens.len() != 0 {
            let mut state = String::new();
            let mut letter = String::new();

            let mut begin = 0;
            for index in 0..tokens.len() {
                if tokens[index] == ";" {
                    state = Nfa::tokens_to_states(&tokens[begin..index])
                        .iter()
                        .cloned()
                        .next()
                        .unwrap();

                    begin = index + 1;
                } else if tokens[index] == "->" {
                    letter = tokens[begin..index].iter().cloned().collect();

                    begin = index + 1;
                } else if tokens[index] == "|" {
                    let out_states = Nfa::tokens_to_states(&tokens[begin..index]);

                    transition_function.insert((state.clone(), letter.clone()), out_states.clone());

                    begin = index + 1;
                }

                if index == (tokens.len() - 1) {
                    let out_state = Nfa::tokens_to_states(&tokens[begin..(index + 1)]);

                    transition_function.insert((state.clone(), letter.clone()), out_state.clone());
                }
            }
        }

        transition_function
    }

    fn parse_input(input: &str, nfa: &mut NondeterministicFiniteAutomata) -> Result<(), ()> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.len() != 0 {
            match tokens[0] {
                "help" => Nfa::help(),
                "exit" => return Err(()),
                "states" | "s" => match tokens.iter().nth(1) {
                    Some(operation) => match operation {
                        &"add" => {
                            let states = Nfa::tokens_to_states(&tokens[2..tokens.len()]);
                            nfa.states = nfa.states.union(&states).cloned().collect();
                            println!("{:?}", nfa.states);
                        }
                        &"rm" => {
                            let states = Nfa::tokens_to_states(&tokens[2..tokens.len()]);
                            nfa.states = nfa.states.difference(&states).cloned().collect();
                            println!("{:?}", nfa.states);
                        }
                        x => println!("{} is not a valid operation.", *x),
                    },
                    None => println!("{:?}", nfa.states),
                },
                "alphabet" | "a" => match tokens.iter().nth(1) {
                    Some(operation) => match operation {
                        &"add" => {
                            for token in &tokens[2..tokens.len()] {
                                nfa.alphabet.insert(token.to_string());
                            }
                            println!("{:?}", nfa.alphabet);
                        }
                        &"rm" => {
                            for token in &tokens[2..tokens.len()] {
                                nfa.alphabet.remove(&token.to_string());
                            }
                            println!("{:?}", nfa.alphabet);
                        }
                        x => println!("{} is not a valid operation.", *x),
                    },
                    None => println!("{:?}", nfa.alphabet),
                },
                "transition" | "t" => match tokens.iter().nth(1) {
                    Some(operation) => match operation {
                        &"add" => {
                            let transitions = Nfa::tokens_to_transitions(&tokens[2..tokens.len()]);

                            for ((state, letter), out_state) in &transitions {
                                nfa.transition_function
                                    .insert((state.clone(), letter.clone()), out_state.clone());
                            }

                            println!("{}", nfa);
                        }
                        &"rm" => {
                            let transitions = Nfa::tokens_to_transitions(&tokens[2..tokens.len()]);

                            for ((state, letter), _) in &transitions {
                                nfa.transition_function
                                    .remove(&(state.clone(), letter.clone()));
                            }

                            println!("{}", nfa);
                        }
                        x => println!("{} is not a valid operation.", *x),
                    },
                    None => println!("{}", nfa),
                },
                "start_state" | "ss" => match tokens.iter().nth(1) {
                    Some(operation) => match operation {
                        &"set" => {
                            nfa.start_state = tokens[2].to_string();
                            println!("{}", nfa.start_state);
                        }
                        x => println!("{} is not a valid operation.", *x),
                    },
                    None => println!("{}", nfa.start_state),
                },
                "accept_states" | "as" => match tokens.iter().nth(1) {
                    Some(operation) => match operation {
                        &"add" => {
                            let states = Nfa::tokens_to_states(&tokens[2..tokens.len()]);
                            nfa.accept_states = nfa.accept_states.union(&states).cloned().collect();
                            println!("{:?}", nfa.accept_states);
                        }
                        &"rm" => {
                            let states = Nfa::tokens_to_states(&tokens[2..tokens.len()]);
                            nfa.accept_states =
                                nfa.accept_states.difference(&states).cloned().collect();
                            println!("{:?}", nfa.accept_states);
                        }
                        x => println!("{} is not a valid operation.", *x),
                    },
                    None => println!("{:?}", nfa.accept_states),
                },
                x => {
                    println!("unknown command: {}", x);
                }
            }
        }

        Ok(())
    }

    pub fn run(nfa: &mut NondeterministicFiniteAutomata) {
        Nfa::greatings();

        loop {
            match Nfa::wait_for_input() {
                Ok(input) => match Nfa::parse_input(&input, nfa) {
                    Ok(_) => (),
                    Err(_) => return,
                },
                Err(error) => println!("{}", error),
            };
        }
    }

    pub fn new_nfa() -> NondeterministicFiniteAutomata {
        NondeterministicFiniteAutomata {
            states: BTreeSet::new(),
            alphabet: BTreeSet::new(),
            transition_function: BTreeMap::new(),
            start_state: String::new(),
            accept_states: BTreeSet::new(),
        }
    }
}

mod test {
    #[test]
    fn tokens_to_state() {
        let input = "";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Nfa::tokens_to_states(&tokens);

        let answer = [].iter().cloned().collect();

        assert_eq!(states, answer);

        let input = "q0";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Nfa::tokens_to_states(&tokens);

        let answer = [String::from("q0")].iter().cloned().collect();

        assert_eq!(states, answer);

        let input = "q0 q1";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Nfa::tokens_to_states(&tokens);

        let answer = [String::from("q0"), String::from("q1")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(states, answer);

        let input = "() ()";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Nfa::tokens_to_states(&tokens);

        let answer = [String::from("()"), String::from("()")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(states, answer);

        let input = "(q1) ((q2))";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Nfa::tokens_to_states(&tokens);

        let answer = [String::from("(q1)"), String::from("((q2))")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(states, answer);

        let input = "(q1, q2) ((q2))";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Nfa::tokens_to_states(&tokens);

        let answer = [String::from("(q1, q2)"), String::from("((q2))")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(states, answer);

        let input = "(q1, (q2, (q3))) ((q2), q1)";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Nfa::tokens_to_states(&tokens);

        let answer = [String::from("((q2), q1)"), String::from("(q1, (q2, (q3)))")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(states, answer);
    }
}
