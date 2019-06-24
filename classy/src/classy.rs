use crate::dfa::Dfa;
use crate::nfa::Nfa;
use crate::rg::Rg;
use formally_cool::regular_languages::{
    DeterministicFiniteAutomata, NondeterministicFiniteAutomata, RegularGrammar,
};
use std::collections::{BTreeMap, HashMap};
use std::io;
use std::io::Write;

pub struct Classy {
    id_to_dfa: HashMap<String, DeterministicFiniteAutomata>,
    id_to_nfa: HashMap<String, NondeterministicFiniteAutomata>,
    id_to_rg: HashMap<String, RegularGrammar>,
}

impl Classy {
    pub fn new() -> Self {
        let mut id_to_dfa = HashMap::new();

        let mut hash = BTreeMap::new();

        hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
        hash.insert((String::from("q1"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q1"), String::from("b")), String::from("q1"));

        let automata = DeterministicFiniteAutomata {
            states: ["q0".to_string(), "q1".to_string()]
                .iter()
                .cloned()
                .collect(),
            alphabet: ["a".to_string(), "b".to_string()].iter().cloned().collect(),
            transition_function: hash,
            start_state: String::from("q0"),
            accept_states: [String::from("q0")].iter().cloned().collect(),
        };

        id_to_dfa.insert("dfa_ends_with_a".to_string(), automata);

        let mut hash = BTreeMap::new();

        hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
        hash.insert((String::from("q1"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q1"), String::from("b")), String::from("q1"));

        let automata2 = DeterministicFiniteAutomata {
            states: ["q0".to_string(), "q1".to_string()]
                .iter()
                .cloned()
                .collect(),
            alphabet: ["a".to_string(), "b".to_string()].iter().cloned().collect(),
            transition_function: hash,
            start_state: String::from("q1"),
            accept_states: [String::from("q1")].iter().cloned().collect(),
        };

        id_to_dfa.insert("dfa_ends_with_b".to_string(), automata2);

        Classy {
            id_to_dfa: id_to_dfa,
            id_to_nfa: HashMap::new(),
            id_to_rg: HashMap::new(),
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
            "[expression] => new [type]",
            "Create empty object of [type].",
            width = width
        );
        println!(
            "{:<width$}{}",
            "[expression] => read [type] [file_name]",
            "Create new object from file.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "[expression] => minimize [id]",
            "Create minimized DFA from [id].",
            width = width
        );
        println!(
            "{:<width$}{}",
            "[expression] => union [id] [id]",
            "Create union DFA from [id] and [id].",
            width = width
        );
        println!(
            "{:<width$}{}",
            "[expression] => intersection [id] [id]",
            "Create intersection DFA from [id] and [id].",
            width = width
        );
        println!(
            "{:<width$}{}",
            "[expression] => to_rg [id]",
            "Transform [id] DFA to RG.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "[expression] => to_nfa [id]",
            "Transform [id] to NFA.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "[expression] => to_dfa [id]",
            "Transform [id] to DFA.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "compute [id] [string]",
            "Checks if [id] dfa accepts [string] or not.",
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
            "[type] => dfa | nfa | rg",
            "Types available.",
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

    fn parse_expression(&mut self, tokens: &[&str]) {
        match tokens.iter().nth(0) {
            Some(id) => match tokens.iter().nth(1) {
                Some(assingment) => {
                    if *assingment == "=" {
                        match tokens.iter().nth(2) {
                            Some(&"new") => match tokens.iter().nth(3) {
                                Some(x) => {
                                    if *x == "dfa" {
                                        let mut dfa = Dfa::new_dfa();
                                        Dfa::run(&mut dfa);
                                        println!("{}", dfa);
                                        self.id_to_dfa.insert(id.to_string(), dfa);
                                    } else if *x == "nfa" {
                                        let mut nfa = Nfa::new_nfa();
                                        Nfa::run(&mut nfa);
                                        println!("{}", nfa);
                                        self.id_to_nfa.insert(id.to_string(), nfa);
                                    } else if *x == "rg" {
                                        let mut rg = Rg::new_rg();
                                        Rg::run(&mut rg);
                                        println!("{}", rg);
                                        self.id_to_rg.insert(id.to_string(), rg);
                                    } else {
                                        println!("unknown type: {}.", *x);
                                    }
                                }
                                None => println!("Expected type after new for {}.", *id),
                            },
                            Some(&"minimize") => match tokens.iter().nth(3) {
                                Some(rhs_id) => {
                                    if self.id_to_dfa.contains_key(&rhs_id.to_string()) {
                                        match self.id_to_dfa.get(&rhs_id.to_string()) {
                                            Some(dfa) => {
                                                let minimized = dfa.minimize();
                                                println!("{}", minimized);
                                                self.id_to_dfa.insert(id.to_string(), minimized);
                                            }
                                            None => (),
                                        }
                                    } else {
                                        println!("{} is not a valid DFA id.", rhs_id)
                                    }
                                }
                                None => println!("Expected DFA id after minimize for {}.", *id),
                            },
                            Some(&"union") => match tokens.iter().nth(3) {
                                Some(lhs_id) => {
                                    if self.id_to_dfa.contains_key(&lhs_id.to_string()) {
                                        match self.id_to_dfa.get(&lhs_id.to_string()) {
                                            Some(lhs_dfa) => match tokens.iter().nth(4) {
                                                Some(rhs_id) => {
                                                    if self
                                                        .id_to_dfa
                                                        .contains_key(&rhs_id.to_string())
                                                    {
                                                        match self
                                                            .id_to_dfa
                                                            .get(&rhs_id.to_string())
                                                        {
                                                            Some(rhs_dfa) => {
                                                                let union = lhs_dfa.union(rhs_dfa);
                                                                println!("{}", union);
                                                                self.id_to_dfa
                                                                    .insert(id.to_string(), union);
                                                            }
                                                            None => (),
                                                        }
                                                    } else {
                                                        println!(
                                                            "{} is not a valid DFA id.",
                                                            rhs_id
                                                        )
                                                    }
                                                }
                                                None => println!("Expected id"),
                                            },
                                            None => (),
                                        }
                                    } else {
                                        println!("{} is not a valid DFA id.", lhs_id)
                                    }
                                }
                                None => println!("Expected DFA id after union for {}.", *id),
                            },
                            Some(&"intersection") => match tokens.iter().nth(3) {
                                Some(lhs_id) => {
                                    if self.id_to_dfa.contains_key(&lhs_id.to_string()) {
                                        match self.id_to_dfa.get(&lhs_id.to_string()) {
                                            Some(lhs_dfa) => match tokens.iter().nth(4) {
                                                Some(rhs_id) => {
                                                    if self
                                                        .id_to_dfa
                                                        .contains_key(&rhs_id.to_string())
                                                    {
                                                        match self
                                                            .id_to_dfa
                                                            .get(&rhs_id.to_string())
                                                        {
                                                            Some(rhs_dfa) => {
                                                                let intersection =
                                                                    lhs_dfa.intersection(rhs_dfa);
                                                                println!("{}", intersection);
                                                                self.id_to_dfa.insert(
                                                                    id.to_string(),
                                                                    intersection,
                                                                );
                                                            }
                                                            None => (),
                                                        }
                                                    } else {
                                                        println!(
                                                            "{} is not a valid DFA id.",
                                                            rhs_id
                                                        )
                                                    }
                                                }
                                                None => println!("Expected id"),
                                            },
                                            None => (),
                                        }
                                    } else {
                                        println!("{} is not a valid DFA id.", lhs_id)
                                    }
                                }
                                None => println!("Expected DFA id after intersection for {}.", *id),
                            },
                            Some(&"to_rg") => match tokens.iter().nth(3) {
                                Some(rhs_id) => {
                                    if self.id_to_dfa.contains_key(&rhs_id.to_string()) {
                                        match self.id_to_dfa.get(&rhs_id.to_string()) {
                                            Some(dfa) => {
                                                let rg = RegularGrammar::from(dfa);
                                                println!("{}", rg);
                                                self.id_to_rg.insert(id.to_string(), rg);
                                            }
                                            None => (),
                                        }
                                    } else {
                                        println!("{} is not a valid DFA id.", rhs_id)
                                    }
                                }
                                None => println!("Expected DFA id after to_rg for {}.", *id),
                            },
                            Some(&"to_nfa") => match tokens.iter().nth(3) {
                                Some(rhs_id) => {
                                    if self.id_to_rg.contains_key(&rhs_id.to_string()) {
                                        match self.id_to_rg.get(&rhs_id.to_string()) {
                                            Some(rg) => {
                                                let nfa = NondeterministicFiniteAutomata::from(rg);
                                                println!("{}", nfa);
                                                self.id_to_nfa.insert(id.to_string(), nfa);
                                            }
                                            None => (),
                                        }
                                    } else {
                                        println!("{} is not a valid RG id.", rhs_id)
                                    }
                                }
                                None => println!("Expected RG id after to_nfa for {}.", *id),
                            },
                            Some(&"to_dfa") => match tokens.iter().nth(3) {
                                Some(rhs_id) => {
                                    if self.id_to_nfa.contains_key(&rhs_id.to_string()) {
                                        match self.id_to_nfa.get(&rhs_id.to_string()) {
                                            Some(nfa) => {
                                                let dfa = DeterministicFiniteAutomata::from(nfa);
                                                println!("{}", dfa);
                                                self.id_to_dfa.insert(id.to_string(), dfa);
                                            }
                                            None => (),
                                        }
                                    } else {
                                        println!("{} is not a valid NFA id.", rhs_id)
                                    }
                                }
                                None => println!("Expected RG id after to_dfa for {}.", *id),
                            },
                            Some(&"read") => match tokens.iter().nth(3) {
                                Some(x) => {
                                    if *x == "dfa" {
                                        match tokens.iter().nth(4) {
                                            Some(file_name) => {
                                                match std::fs::File::open(file_name) {
                                                    Ok(file) => {
                                                        let reader = std::io::BufReader::new(file);
                                                        match serde_yaml::from_reader(reader) {
                                                            Ok(dfa) => {
                                                                println!("{}", dfa);
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
                                    } else if *x == "nfa" {
                                        match tokens.iter().nth(4) {
                                            Some(file_name) => {
                                                match std::fs::File::open(file_name) {
                                                    Ok(file) => {
                                                        let reader = std::io::BufReader::new(file);
                                                        match serde_yaml::from_reader(reader) {
                                                            Ok(nfa) => {
                                                                println!("{}", nfa);
                                                                self.id_to_nfa
                                                                    .insert(id.to_string(), nfa);
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
                                    } else if *x == "rg" {
                                        match tokens.iter().nth(4) {
                                            Some(file_name) => {
                                                match std::fs::File::open(file_name) {
                                                    Ok(file) => {
                                                        let reader = std::io::BufReader::new(file);
                                                        match serde_yaml::from_reader(reader) {
                                                            Ok(rg) => {
                                                                println!("{}", rg);
                                                                self.id_to_rg
                                                                    .insert(id.to_string(), rg);
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
                                None => println!("Expected type after new for {}.", *id),
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
        }
    }

    fn parse_input(&mut self, input: &str) {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.len() != 0 {
            match tokens[0] {
                "help" => Classy::help(),
                "exit" => std::process::exit(0),
                "let" => {
                    self.parse_expression(&tokens[1..tokens.len()]);
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
                        } else if self.id_to_nfa.contains_key(&id.to_string()) {
                            match tokens.iter().nth(2) {
                                Some(file_name) => match self.id_to_nfa.get(&id.to_string()) {
                                    Some(nfa) => match std::fs::File::create(file_name) {
                                        Ok(file) => {
                                            let writer = std::io::BufWriter::new(file);
                                            match serde_yaml::to_writer(writer, &nfa) {
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
                        } else if self.id_to_rg.contains_key(&id.to_string()) {
                            match tokens.iter().nth(2) {
                                Some(file_name) => match self.id_to_rg.get(&id.to_string()) {
                                    Some(rg) => match std::fs::File::create(file_name) {
                                        Ok(file) => {
                                            let writer = std::io::BufWriter::new(file);
                                            match serde_yaml::to_writer(writer, &rg) {
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
                        } else if self.id_to_nfa.contains_key(&id.to_string()) {
                            match self.id_to_nfa.get_mut(&id.to_string()) {
                                Some(mut nfa) => {
                                    Nfa::run(&mut nfa);
                                    println!("{}", nfa);
                                }
                                None => (),
                            }
                        } else if self.id_to_rg.contains_key(&id.to_string()) {
                            match self.id_to_rg.get_mut(&id.to_string()) {
                                Some(mut rg) => {
                                    Rg::run(&mut rg);
                                    println!("{}", rg);
                                }
                                None => (),
                            }
                        } else {
                            println!("unknown id: {}", id);
                        }
                    }
                    None => println!("Expected id after edit."),
                },
                "compute" => match tokens.iter().nth(1) {
                    Some(id) => {
                        if self.id_to_dfa.contains_key(&id.to_string()) {
                            match self.id_to_dfa.get(&id.to_string()) {
                                Some(dfa) => match tokens.iter().nth(2) {
                                    Some(string) => {
                                        println!("{}", dfa.compute(&string.to_string()));
                                    }
                                    None => {
                                        println!("Expected string after id");
                                    }
                                },
                                None => (),
                            }
                        } else {
                            println!("unknown dfa: {}", id);
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
                    } else if self.id_to_nfa.contains_key(&x.to_string()) {
                        match self.id_to_nfa.get(&x.to_string()) {
                            Some(nfa) => println!("{}", nfa),
                            None => (),
                        }
                    } else if self.id_to_rg.contains_key(&x.to_string()) {
                        match self.id_to_rg.get(&x.to_string()) {
                            Some(rg) => println!("{}", rg),
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
