use std::collections::HashMap;
use std::collections::HashSet;

fn state_to_symbol(state: &String) -> String {
    String::from("<") + state + ">"
}

fn symbol_to_state(symbol: &String) -> String {
    symbol[1..(symbol.len() - 1)].to_string()
}

#[derive(Debug, Clone)]
pub struct Automata {
    pub start_state: String,
    pub transition_function: HashMap<(String, String), String>,
    pub accept_states: HashSet<String>,
}

impl Automata {
    pub fn compute(&self, input: &str) -> bool {
        let mut actual_state = self.start_state.clone();
        for symbol in input.chars() {
            actual_state = self.transition_function[&(actual_state, symbol.to_string())].clone();
        }
        self.accept_states.contains(&actual_state)
    }
}

impl From<&RegularGrammar> for Automata {
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

        Automata {
            start_state: symbol_to_state(&regular_grammar.start_symbol),
            transition_function: transition_function,
            accept_states: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegularGrammar {
    pub start_symbol: String,
    pub productions: HashMap<String, Vec<String>>,
}

impl From<&Automata> for RegularGrammar {
    fn from(automata: &Automata) -> Self {
        let mut productions = HashMap::new();

        for ((state, _), _) in &automata.transition_function {
            productions.insert(state_to_symbol(state), vec![]);
        }

        for ((state, entry_symbol), next_state) in &automata.transition_function {
            match productions.get_mut(&state_to_symbol(state)) {
                Some(x) => x.push(entry_symbol.clone() + &state_to_symbol(next_state)),
                None => (),
            }

            if automata
                .accept_states
                .contains(&state_to_symbol(next_state))
            {
                match productions.get_mut(&state_to_symbol(state)) {
                    Some(x) => x.push(entry_symbol.clone()),
                    None => (),
                }
            }
        }

        RegularGrammar {
            start_symbol: state_to_symbol(&automata.start_state),
            productions: productions,
        }
    }
}
