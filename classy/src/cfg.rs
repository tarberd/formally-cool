use formally_cool::context_free_languages::ContextFreeGrammar;
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::io::Write;

pub struct Cfg {}

impl Cfg {
    pub fn new() -> Self {
        Cfg {}
    }

    fn greatings() {
        println!("{}", "Welcome to the CFG tool.");
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
            "(r)rules add <S> => a<A> | b | b<B> ...",
            "Print start variable.",
            width = width
        );
        println!(
            "{:<width$}{}",
            "(r)rules rm <S> => a<A> | b | b<B> ...",
            "Print start variable.",
            width = width
        );
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
        print!("{}", "cfg> ");

        io::stdout().flush().expect("Error flushing stdout");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Ok(input.trim().to_string()),
            Err(error) => Err(error),
        }
    }

    fn tokens_to_variables(tokens: &[&str]) -> BTreeSet<String> {
        let mut variables = BTreeSet::new();

        if tokens.len() != 0 {
            let mut variable = String::new();

            let mut open_variable = false;

            for index in 0..tokens.len() {
                let first_letter = tokens[index].get(0..1).unwrap();
                let end = tokens[index].len();
                let last_letter = tokens[index].get((end - 1)..end).unwrap();

                if open_variable == false {
                    variable = variable + tokens[index];
                } else {
                    variable = variable + " " + tokens[index];
                }

                if first_letter == "<" {
                    open_variable = true;
                }

                if last_letter == ">" {
                    variables.insert(variable.clone());
                    variable = String::new();
                    open_variable = false;
                }
            }
        }

        variables
    }

    fn tokens_to_rules(tokens: &[&str]) -> BTreeMap<String, BTreeSet<Vec<String>>> {
        let mut rules = BTreeMap::new();

        if tokens.len() >= 3 {
            if tokens[1] == "=>" {
                let source_variable = tokens[0].to_string();

                let mut out_set = BTreeSet::new();
                let mut vec = vec![];

                let mut variable = String::new();
                let mut is_open_variable = false;

                for index in 2..tokens.len() {
                    if tokens[index] != "|" {
                        for letter in tokens[index].chars() {
                            if letter == '<' {
                                variable = String::new();
                                is_open_variable = true;
                            } else if letter == '>' {
                                variable = variable + ">";
                                is_open_variable = false;
                                vec.push(variable.to_string());
                            }

                            if is_open_variable {
                                variable = variable + &letter.to_string();
                            } else if letter != '<' && letter != '>' {
                                vec.push(letter.to_string());
                            }
                        }
                    }

                    if index + 1 == tokens.len() || tokens[index] == "|" {
                        out_set.insert(vec);
                        vec = vec![];
                    }
                }

                rules.insert(source_variable, out_set);
            }
        }

        rules
    }

    fn parse_input(input: &str, cfg: &mut ContextFreeGrammar) -> Result<(), ()> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.len() != 0 {
            match tokens[0] {
                "help" => Cfg::help(),
                "exit" => return Err(()),
                "variables" | "v" => match tokens.iter().nth(1) {
                    Some(&"add") => {
                        let variables = Cfg::tokens_to_variables(&tokens[2..tokens.len()]);
                        cfg.variables = cfg.variables.union(&variables).cloned().collect();
                        println!("{:?}", cfg.variables);
                    }
                    Some(&"rm") => {
                        let variables = Cfg::tokens_to_variables(&tokens[2..tokens.len()]);
                        cfg.variables = cfg.variables.difference(&variables).cloned().collect();
                        println!("{:?}", cfg.variables);
                    }
                    Some(other) => println!("unknown command: {}", other),
                    None => println!("{:?}", cfg.variables),
                },
                "terminals" | "t" => match tokens.iter().nth(1) {
                    Some(&"add") => {
                        for token in &tokens[2..tokens.len()] {
                            cfg.terminals.insert(token.to_string());
                        }
                        println!("{:?}", cfg.terminals);
                    }
                    Some(&"rm") => {
                        for token in &tokens[2..tokens.len()] {
                            cfg.terminals.remove(&token.to_string());
                        }
                        println!("{:?}", cfg.terminals);
                    }
                    Some(other) => println!("unknown command: {}", other),
                    None => println!("{:?}", cfg.variables),
                },
                "rules" | "r" => match tokens.iter().nth(1) {
                    Some(&"add") => {
                        let rules = Cfg::tokens_to_rules(&tokens[2..tokens.len()]);

                        for (variable, productions) in &rules {
                            match cfg.rules.get_mut(variable) {
                                Some(old_productions) => {
                                    *old_productions =
                                        old_productions.union(&productions).cloned().collect();
                                }
                                None => {
                                    cfg.rules.insert(variable.clone(), productions.clone());
                                }
                            }
                        }

                        println!("{}", cfg);
                    }
                    Some(&"rm") => {
                        let rules = Cfg::tokens_to_rules(&tokens[2..tokens.len()]);

                        for (variable, productions) in &rules {
                            match cfg.rules.get_mut(variable) {
                                Some(old_productions) => {
                                    *old_productions =
                                        old_productions.difference(&productions).cloned().collect();
                                }
                                None => (),
                            }
                        }

                        println!("{}", cfg);
                    }
                    Some(other) => println!("unknown command: {}", other),
                    None => println!("{}", cfg),
                },
                "start_variable" | "sv" => match tokens.iter().nth(1) {
                    Some(&"set") => {
                        cfg.start_variable = tokens[2].to_string();
                        println!("{}", cfg.start_variable);
                    }
                    Some(other) => println!("unknown command: {}", other),
                    None => println!("{:?}", cfg.variables),
                },
                x => {
                    println!("unknown command: {}", x);
                }
            }
        }

        Ok(())
    }

    pub fn run(cfg: &mut ContextFreeGrammar) {
        Cfg::greatings();

        loop {
            match Cfg::wait_for_input() {
                Ok(input) => match Cfg::parse_input(&input, cfg) {
                    Ok(_) => (),
                    Err(_) => return,
                },
                Err(error) => println!("{}", error),
            };
        }
    }

    pub fn new_cfg() -> ContextFreeGrammar {
        ContextFreeGrammar::new()
    }
}

mod test {
    #[test]
    fn test_tokens_to_rules() {
        use std::collections::BTreeMap;

        let input = "<S> => a";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let rules = super::Cfg::tokens_to_rules(&tokens);

        let mut answer = BTreeMap::new();

        answer.insert(
            String::from("<S>"),
            [vec![String::from("a")]].iter().cloned().collect(),
        );

        assert_eq!(rules, answer);

        let input = "<S> => abcd";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let rules = super::Cfg::tokens_to_rules(&tokens);

        let mut answer = BTreeMap::new();

        answer.insert(
            String::from("<S>"),
            [vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
                String::from("d"),
            ]]
            .iter()
            .cloned()
            .collect(),
        );

        assert_eq!(rules, answer);

        let input = "<S> => ab<S>cd";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let rules = super::Cfg::tokens_to_rules(&tokens);

        let mut answer = BTreeMap::new();

        answer.insert(
            String::from("<S>"),
            [vec![
                String::from("a"),
                String::from("b"),
                String::from("<S>"),
                String::from("c"),
                String::from("d"),
            ]]
            .iter()
            .cloned()
            .collect(),
        );

        assert_eq!(rules, answer);

        let input = "<S> => <S><S><S>";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let rules = super::Cfg::tokens_to_rules(&tokens);

        let mut answer = BTreeMap::new();

        answer.insert(
            String::from("<S>"),
            [vec![
                String::from("<S>"),
                String::from("<S>"),
                String::from("<S>"),
            ]]
            .iter()
            .cloned()
            .collect(),
        );

        assert_eq!(rules, answer);

        let input = "<S> => ab<S>ab<S>ab<S>ab";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let rules = super::Cfg::tokens_to_rules(&tokens);

        let mut answer = BTreeMap::new();

        answer.insert(
            String::from("<S>"),
            [vec![
                String::from("a"),
                String::from("b"),
                String::from("<S>"),
                String::from("a"),
                String::from("b"),
                String::from("<S>"),
                String::from("a"),
                String::from("b"),
                String::from("<S>"),
                String::from("a"),
                String::from("b"),
            ]]
            .iter()
            .cloned()
            .collect(),
        );

        assert_eq!(rules, answer);

        let input = "<S> => ab<S>ab<S>ab<S>ab | ab<S>ab<S>ab<S> | ab<S>ab<S>ab";
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let rules = super::Cfg::tokens_to_rules(&tokens);

        let mut answer = BTreeMap::new();

        answer.insert(
            String::from("<S>"),
            [
                vec![
                    String::from("a"),
                    String::from("b"),
                    String::from("<S>"),
                    String::from("a"),
                    String::from("b"),
                    String::from("<S>"),
                    String::from("a"),
                    String::from("b"),
                ],
                vec![
                    String::from("a"),
                    String::from("b"),
                    String::from("<S>"),
                    String::from("a"),
                    String::from("b"),
                    String::from("<S>"),
                    String::from("a"),
                    String::from("b"),
                    String::from("<S>"),
                ],
                vec![
                    String::from("a"),
                    String::from("b"),
                    String::from("<S>"),
                    String::from("a"),
                    String::from("b"),
                    String::from("<S>"),
                    String::from("a"),
                    String::from("b"),
                    String::from("<S>"),
                    String::from("a"),
                    String::from("b"),
                ],
            ]
            .iter()
            .cloned()
            .collect(),
        );

        assert_eq!(rules, answer);
    }
}
