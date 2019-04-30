use std::collections::HashMap;
use std::collections::HashSet;

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

#[derive(Debug, Clone)]
pub struct RegularGrammar {
    pub start_symbol: String,
    pub productions: HashMap<String, Vec<String>>,
}

fn state_to_symbol(state: &String) -> String {
    String::from("<") + state + ">"
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
