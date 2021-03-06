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
            "(t)transition add [state] ; [letter] -> [state] | [state] ; [letter] -> [state] | ...",
            "\n\tAdd list of transitions separeted by '|'.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(t)transition rm [state] ; [letter] -> [state] | [state] ; [letter] -> [state] | ...",
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
        print!("{}", "dfa> ");

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

    fn tokens_to_transitions(tokens: &[&str]) -> BTreeMap<(String, String), String> {
        let mut transition_function = BTreeMap::new();

        if tokens.len() != 0 {
            let mut state = String::new();
            let mut letter = String::new();

            let mut begin = 0;
            for index in 0..tokens.len() {
                if tokens[index] == ";" {
                    state = Dfa::tokens_to_states(&tokens[begin..index])
                        .iter()
                        .cloned()
                        .next()
                        .unwrap();

                    begin = index + 1;
                } else if tokens[index] == "->" {
                    letter = tokens[begin..index].iter().cloned().collect();

                    begin = index + 1;
                } else if tokens[index] == "|" {
                    let out_state = Dfa::tokens_to_states(&tokens[begin..index])
                        .iter()
                        .cloned()
                        .next()
                        .unwrap();

                    transition_function.insert((state.clone(), letter.clone()), out_state.clone());

                    begin = index + 1;
                }

                if index == (tokens.len() - 1) {
                    let out_state = Dfa::tokens_to_states(&tokens[begin..(index + 1)])
                        .iter()
                        .cloned()
                        .next()
                        .unwrap();

                    transition_function.insert((state.clone(), letter.clone()), out_state.clone());
                }
            }
        }

        transition_function
    }

    fn parse_input(input: &str, dfa: &mut DeterministicFiniteAutomata) -> Result<(), ()> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.len() != 0 {
            match tokens[0] {
                "help" => Dfa::help(),
                "exit" => return Err(()),
                "states" | "s" => match tokens.iter().nth(1) {
                    Some(operation) => match operation {
                        &"add" => {
                            let states = Dfa::tokens_to_states(&tokens[2..tokens.len()]);
                            dfa.states = dfa.states.union(&states).cloned().collect();
                            println!("{:?}", dfa.states);
                        }
                        &"rm" => {
                            let states = Dfa::tokens_to_states(&tokens[2..tokens.len()]);
                            dfa.states = dfa.states.difference(&states).cloned().collect();
                            println!("{:?}", dfa.states);
                        }
                        x => println!("{} is not a valid operation.", *x),
                    },
                    None => println!("{:?}", dfa.states),
                },
                "alphabet" | "a" => match tokens.iter().nth(1) {
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
                "transition" | "t" => match tokens.iter().nth(1) {
                    Some(operation) => match operation {
                        &"add" => {
                            let transitions = Dfa::tokens_to_transitions(&tokens[2..tokens.len()]);

                            for ((state, letter), out_state) in &transitions {
                                dfa.transition_function
                                    .insert((state.clone(), letter.clone()), out_state.clone());
                            }

                            println!("{}", dfa);
                        }
                        &"rm" => {
                            let transitions = Dfa::tokens_to_transitions(&tokens[2..tokens.len()]);

                            for ((state, letter), _) in &transitions {
                                dfa.transition_function
                                    .remove(&(state.clone(), letter.clone()));
                            }

                            println!("{}", dfa);
                        }
                        x => println!("{} is not a valid operation.", *x),
                    },
                    None => println!("{}", dfa),
                },
                "start_state" | "ss" => match tokens.iter().nth(1) {
                    Some(operation) => match operation {
                        &"set" => {
                            dfa.start_state = tokens[2].to_string();
                            println!("{}", dfa.start_state);
                        }
                        x => println!("{} is not a valid operation.", *x),
                    },
                    None => println!("{}", dfa.start_state),
                },
                "accept_states" | "as" => match tokens.iter().nth(1) {
                    Some(operation) => match operation {
                        &"add" => {
                            let states = Dfa::tokens_to_states(&tokens[2..tokens.len()]);
                            dfa.accept_states = dfa.accept_states.union(&states).cloned().collect();
                            println!("{:?}", dfa.accept_states);
                        }
                        &"rm" => {
                            let states = Dfa::tokens_to_states(&tokens[2..tokens.len()]);
                            dfa.accept_states =
                                dfa.accept_states.difference(&states).cloned().collect();
                            println!("{:?}", dfa.accept_states);
                        }
                        x => println!("{} is not a valid operation.", *x),
                    },
                    None => println!("{:?}", dfa.accept_states),
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

mod test {

    #[test]
    fn tokens_to_state() {
        let input = "";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Dfa::tokens_to_states(&tokens);

        let answer = [].iter().cloned().collect();

        assert_eq!(states, answer);

        let input = "q0";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Dfa::tokens_to_states(&tokens);

        let answer = [String::from("q0")].iter().cloned().collect();

        assert_eq!(states, answer);

        let input = "q0 q1";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Dfa::tokens_to_states(&tokens);

        let answer = [String::from("q0"), String::from("q1")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(states, answer);

        let input = "() ()";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Dfa::tokens_to_states(&tokens);

        let answer = [String::from("()"), String::from("()")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(states, answer);

        let input = "(q1) ((q2))";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Dfa::tokens_to_states(&tokens);

        let answer = [String::from("(q1)"), String::from("((q2))")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(states, answer);

        let input = "(q1, q2) ((q2))";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Dfa::tokens_to_states(&tokens);

        let answer = [String::from("(q1, q2)"), String::from("((q2))")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(states, answer);

        let input = "(q1, (q2, (q3))) ((q2), q1)";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let states = super::Dfa::tokens_to_states(&tokens);

        let answer = [String::from("((q2), q1)"), String::from("(q1, (q2, (q3)))")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(states, answer);
    }

    #[test]
    fn tokens_to_transitions() {
        use std::collections::BTreeMap;

        let input = "q0 ; a -> q0";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let transitions = super::Dfa::tokens_to_transitions(&tokens);

        let mut answer = BTreeMap::new();

        answer.insert((String::from("q0"), String::from("a")), String::from("q0"));

        assert_eq!(transitions, answer);

        let input = "(q0, q2) ; a -> (q1, q3)";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let transitions = super::Dfa::tokens_to_transitions(&tokens);

        let mut answer = BTreeMap::new();

        answer.insert(
            (String::from("(q0, q2)"), String::from("a")),
            String::from("(q1, q3)"),
        );

        assert_eq!(transitions, answer);

        let input = "(q0, q2) ; a -> (q1, q3) | (q0, q2) ; b -> (q1, q3)";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let transitions = super::Dfa::tokens_to_transitions(&tokens);

        let mut answer = BTreeMap::new();

        answer.insert(
            (String::from("(q0, q2)"), String::from("a")),
            String::from("(q1, q3)"),
        );
        answer.insert(
            (String::from("(q0, q2)"), String::from("b")),
            String::from("(q1, q3)"),
        );

        assert_eq!(transitions, answer);
    }
}
