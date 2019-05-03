use std::collections::HashMap;
use std::collections::HashSet;

use super::regular_grammar::RegularGrammar;

use serde::{Deserialize, Serialize};

fn symbol_to_state(symbol: &String) -> String {
    symbol[1..(symbol.len() - 1)].to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NondeterministicFiniteAutomata {
    pub start_state: String,
    pub transition_function: HashMap<(String, String), HashSet<String>>,
    pub accept_states: HashSet<String>,
}

impl From<&RegularGrammar> for NondeterministicFiniteAutomata {
    fn from(regular_grammar: &RegularGrammar) -> Self {
        let mut transition_function = HashMap::new();

        let mut accept_states = HashSet::new();

        let final_state = String::from("accept");

        accept_states.insert(final_state.clone());

        for (symbol, set) in &regular_grammar.productions {
            for derivation in set {
                let key = (symbol_to_state(symbol), derivation[0..1].to_string());

                if !transition_function.contains_key(&key) {
                    transition_function.insert(key.clone(), HashSet::new());
                }

                if derivation.len() == 1 {
                    match transition_function.get_mut(&key) {
                        Some(set) => set.insert(final_state.clone()),
                        None => false,
                    };
                } else {
                    match transition_function.get_mut(&key) {
                        Some(set) => set.insert(symbol_to_state(&derivation[1..].to_string())),
                        None => false,
                    };
                }
            }
        }

        NondeterministicFiniteAutomata {
            start_state: symbol_to_state(&regular_grammar.start_symbol),
            transition_function: transition_function,
            accept_states: accept_states,
        }
    }
}
