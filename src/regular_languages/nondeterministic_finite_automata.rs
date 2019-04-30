use std::collections::HashMap;
use std::collections::HashSet;

use super::regular_gramar::RegularGrammar;

fn symbol_to_state(symbol: &String) -> String {
    symbol[1..(symbol.len() - 1)].to_string()
}

#[derive(Debug, Clone)]
pub struct NondeterministicFiniteAutomata {
    pub start_state: String,
    pub transition_function: HashMap<(String, String), String>,
    pub accept_states: HashSet<String>,
}

impl NondeterministicFiniteAutomata {
    pub fn compute(&self, input: &str) -> bool {
        let mut actual_state = self.start_state.clone();
        for symbol in input.chars() {
            actual_state = self.transition_function[&(actual_state, symbol.to_string())].clone();
        }
        self.accept_states.contains(&actual_state)
    }
}

impl From<&RegularGrammar> for NondeterministicFiniteAutomata {
    fn from(regular_grammar: &RegularGrammar) -> Self {
        let mut transition_function = HashMap::new();

        let final_state = String::from("accept");

        for (symbol, vec) in &regular_grammar.productions {
            for derivation in vec {
                if derivation.len() == 1 {
                    transition_function.insert(
                        (symbol_to_state(symbol), derivation[0..1].to_string()),
                        final_state.clone(),
                    );
                } else {
                    transition_function.insert(
                        (symbol_to_state(symbol), derivation[0..1].to_string()),
                        symbol_to_state(&derivation[1..].to_string()),
                    );
                }
            }
        }

        NondeterministicFiniteAutomata {
            start_state: symbol_to_state(&regular_grammar.start_symbol),
            transition_function: transition_function,
            accept_states: HashSet::new(),
        }
    }
}
