use super::deterministic_finite_automata::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;

fn state_to_variable(state: &String) -> String {
    String::from("<") + state + ">"
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegularGrammar {
    pub variables: BTreeSet<String>,
    pub terminals: BTreeSet<String>,
    pub rules: BTreeMap<String, BTreeSet<String>>,
    pub start_variable: String,
}

impl fmt::Display for RegularGrammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => ", self.start_variable)?;
        match self.rules.get(&self.start_variable) {
            Some(set) => {
                for s in set {
                    if s != set.iter().last().unwrap() {
                        write!(f, "{} | ", s)?;
                    } else {
                        write!(f, "{}", s)?;
                    }
                }
            }
            None => write!(f, "-")?,
        };
        write!(f, "\n")?;

        for variable in &self.variables {
            if *variable != self.start_variable {
                write!(f, "{} => ", variable)?;
                match self.rules.get(variable) {
                    Some(set) => {
                        for s in set {
                            if s != set.iter().last().unwrap() {
                                write!(f, "{} | ", s)?;
                            } else {
                                write!(f, "{}", s)?;
                            }
                        }
                    }
                    None => write!(f, "-")?,
                };
                if variable != self.variables.iter().last().unwrap() {
                    write!(f, "\n")?;
                }
            }
        }

        Ok(())
    }
}

impl From<&DeterministicFiniteAutomata> for RegularGrammar {
    fn from(automata: &DeterministicFiniteAutomata) -> Self {
        let mut variables = BTreeSet::new();
        let mut terminals = BTreeSet::new();
        let mut rules = BTreeMap::new();

        for letter in &automata.alphabet {
            terminals.insert(letter.clone());
        }

        for state in &automata.states {
            variables.insert(state_to_variable(state));
        }

        for ((state, _), _) in &automata.transition_function {
            rules.insert(state_to_variable(state), BTreeSet::new());
        }

        for ((state, entry_symbol), next_state) in &automata.transition_function {
            match rules.get_mut(&state_to_variable(state)) {
                Some(set) => set.insert(entry_symbol.clone() + &state_to_variable(next_state)),
                None => false,
            };

            if automata.accept_states.contains(&next_state.clone()) {
                match rules.get_mut(&state_to_variable(state)) {
                    Some(set) => set.insert(entry_symbol.clone()),
                    None => false,
                };
            }
        }

        RegularGrammar {
            variables: variables,
            terminals: terminals,
            rules: rules,
            start_variable: state_to_variable(&automata.start_state),
        }
    }
}
