use super::deterministic_finite_automata::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

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

impl RegularGrammar {
    pub fn printTable(&self){
        println!("{:#?}", self.rules);
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
