use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContextFreeGrammar {
    pub variables: BTreeSet<String>,
    pub terminals: BTreeSet<String>,
    pub rules: BTreeMap<String, BTreeSet<Vec<String>>>,
    pub start_variable: String,
}

impl fmt::Display for ContextFreeGrammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => ", self.start_variable)?;
        match self.rules.get(&self.start_variable) {
            Some(set) => {
                for vec in set {
                    if vec != set.iter().last().unwrap() {
                        for string in vec {
                            if string != vec.iter().last().unwrap() {
                                write!(f, "{}", string)?;
                            } else {
                                write!(f, "{} | ", string)?;
                            }
                        }
                    } else {
                        for string in vec {
                            write!(f, "{}", string)?;
                        }
                    }
                }
            }
            None => write!(f, "-")?,
        };
        write!(f, "\n")?;

        let mut variables = self.variables.clone();
        variables.remove(&self.start_variable);
        for variable in &variables {
            write!(f, "{} => ", variable)?;
            match self.rules.get(variable) {
                Some(set) => {
                    for vec in set {
                        if vec != set.iter().last().unwrap() {
                            for string in vec {
                                if string != vec.iter().last().unwrap() {
                                    write!(f, "{}", string)?;
                                } else {
                                    write!(f, "{} | ", string)?;
                                }
                            }
                        } else {
                            for string in vec {
                                write!(f, "{}", string)?;
                            }
                        }
                    }
                }
                None => write!(f, "-")?,
            };
            if variable != variables.iter().last().unwrap() {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl ContextFreeGrammar {
    pub fn new() -> Self {
        ContextFreeGrammar {
            variables: BTreeSet::new(),
            terminals: BTreeSet::new(),
            rules: BTreeMap::new(),
            start_variable: String::new(),
        }
    }

    fn create_new_start_variable(&self) -> Self {
        let mut new_cfg = self.clone();

        for i in 0.. {
            let new_start_variable = String::from("<S_") + &i.to_string() + ">";

            if !new_cfg.variables.contains(&new_start_variable) {
                new_cfg.variables.insert(new_start_variable.clone());
                new_cfg.rules.insert(
                    new_start_variable.clone(),
                    [vec![new_cfg.start_variable]].iter().cloned().collect(),
                );
                new_cfg.start_variable = new_start_variable;
                break;
            }
        }

        new_cfg
    }

    fn remove_epsilon_productions(&self) -> Self {
        let mut cfg = self.clone();

        let mut got_marked_once = BTreeSet::new();
        let mut marked = vec![];

        for variable in &cfg.variables {
            if *variable != cfg.start_variable {
                match cfg.rules.get_mut(variable) {
                    Some(set) => {
                        if set.contains(&vec![String::from("&")]) {
                            set.remove(&vec![String::from("&")]);
                            marked.push(variable.clone());
                            got_marked_once.insert(variable.clone());
                        }
                    }
                    None => (),
                }
            }
        }

        while let Some(marked_variable) = marked.pop() {
            for (variable, set) in cfg.rules.iter_mut() {
                let mut new_productions = BTreeSet::new();

                for production in set.iter() {
                    new_productions.insert(production.clone());

                    if production.contains(&marked_variable) {
                        let mut stack = vec![production.clone()];

                        while let Some(stack_production) = stack.pop() {
                            let last = stack_production.len();
                            for i in 0..last {
                                if stack_production[i] == marked_variable {
                                    let mut new_production = stack_production[0..i].to_vec();
                                    new_production
                                        .extend(stack_production[(i + 1)..last].iter().cloned());

                                    if new_production.contains(&marked_variable) {
                                        stack.push(new_production.clone());
                                    }

                                    if new_production.is_empty() {
                                        if !got_marked_once.contains(variable) {
                                            marked.push(variable.clone());
                                        }
                                    } else {
                                        new_productions.insert(new_production);
                                    }
                                }
                            }
                        }
                    }
                }

                for production in new_productions {
                    set.insert(production.clone());
                }
            }
        }

        cfg
    }

    pub fn chomsky_normal_form(&self) -> Self {
        let cnf = self.clone();

        cnf
    }
}

mod test {
    use super::*;

    fn make_sipser_cfg_example() -> ContextFreeGrammar {
        let mut cfg = ContextFreeGrammar::new();

        cfg.variables = [
            String::from("<S>"),
            String::from("<A>"),
            String::from("<B>"),
        ]
        .iter()
        .cloned()
        .collect();

        cfg.terminals = [String::from("a"), String::from("b")]
            .iter()
            .cloned()
            .collect();

        cfg.rules = [
            (
                String::from("<S>"),
                [
                    vec![
                        String::from("<A>"),
                        String::from("<S>"),
                        String::from("<A>"),
                    ],
                    vec![String::from("a"), String::from("<B>")],
                ]
                .iter()
                .cloned()
                .collect(),
            ),
            (
                String::from("<A>"),
                [vec![String::from("<B>")], vec![String::from("<S>")]]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            (
                String::from("<B>"),
                [vec![String::from("b")], vec![String::from("&")]]
                    .iter()
                    .cloned()
                    .collect(),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        cfg.start_variable = String::from("<S>");

        cfg
    }

    fn make_sipser_cfg_example_with_new_variable() -> ContextFreeGrammar {
        let mut cfg = ContextFreeGrammar::new();

        cfg.variables = [
            String::from("<S_0>"),
            String::from("<S>"),
            String::from("<A>"),
            String::from("<B>"),
        ]
        .iter()
        .cloned()
        .collect();

        cfg.terminals = [String::from("a"), String::from("b")]
            .iter()
            .cloned()
            .collect();

        cfg.rules = [
            (
                String::from("<S_0>"),
                [vec![String::from("<S>")]].iter().cloned().collect(),
            ),
            (
                String::from("<S>"),
                [
                    vec![
                        String::from("<A>"),
                        String::from("<S>"),
                        String::from("<A>"),
                    ],
                    vec![String::from("a"), String::from("<B>")],
                ]
                .iter()
                .cloned()
                .collect(),
            ),
            (
                String::from("<A>"),
                [vec![String::from("<B>")], vec![String::from("<S>")]]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            (
                String::from("<B>"),
                [vec![String::from("b")], vec![String::from("&")]]
                    .iter()
                    .cloned()
                    .collect(),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        cfg.start_variable = String::from("<S_0>");

        cfg
    }

    fn make_sipser_cfg_example_with_removed_epsilon_productions() -> ContextFreeGrammar {
        let mut cfg = ContextFreeGrammar::new();

        cfg.variables = [
            String::from("<S_0>"),
            String::from("<S>"),
            String::from("<A>"),
            String::from("<B>"),
        ]
        .iter()
        .cloned()
        .collect();

        cfg.terminals = [String::from("a"), String::from("b")]
            .iter()
            .cloned()
            .collect();

        cfg.rules = [
            (
                String::from("<S_0>"),
                [vec![String::from("<S>")]].iter().cloned().collect(),
            ),
            (
                String::from("<S>"),
                [
                    vec![
                        String::from("<A>"),
                        String::from("<S>"),
                        String::from("<A>"),
                    ],
                    vec![String::from("a"), String::from("<B>")],
                    vec![String::from("a")],
                    vec![String::from("<S>"), String::from("<A>")],
                    vec![String::from("<A>"), String::from("<S>")],
                    vec![String::from("<S>")],
                ]
                .iter()
                .cloned()
                .collect(),
            ),
            (
                String::from("<A>"),
                [vec![String::from("<B>")], vec![String::from("<S>")]]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            (
                String::from("<B>"),
                [vec![String::from("b")]].iter().cloned().collect(),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        cfg.start_variable = String::from("<S_0>");

        cfg
    }

    #[test]
    fn test_create_new_start_variable() {
        let cfg = test::make_sipser_cfg_example();

        let correct_cfg = make_sipser_cfg_example_with_new_variable();

        let new_cfg = cfg.create_new_start_variable();

        assert_eq!(new_cfg.terminals, correct_cfg.terminals);
        assert_eq!(new_cfg.variables, correct_cfg.variables);
        assert_eq!(new_cfg.rules, correct_cfg.rules);
        assert_eq!(new_cfg.start_variable, correct_cfg.start_variable);
    }

    #[test]
    fn test_remove_epsilon_productions() {
        let cfg = make_sipser_cfg_example_with_new_variable();

        let correct_cfg = make_sipser_cfg_example_with_removed_epsilon_productions();

        let new_cfg = cfg.remove_epsilon_productions();

        assert_eq!(new_cfg.terminals, correct_cfg.terminals);
        assert_eq!(new_cfg.variables, correct_cfg.variables);
        assert_eq!(new_cfg.rules, correct_cfg.rules);
        assert_eq!(new_cfg.start_variable, correct_cfg.start_variable);
    }
}
