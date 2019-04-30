use std::collections::HashMap;

use super::deterministic_finite_automata::*;

fn state_to_symbol(state: &String) -> String {
    String::from("<") + state + ">"
}

#[derive(Debug, Clone)]
pub struct RegularGrammar {
    pub start_symbol: String,
    pub productions: HashMap<String, Vec<String>>,
}

impl From<&DeterministicFiniteAutomata> for RegularGrammar {
    fn from(automata: &DeterministicFiniteAutomata) -> Self {
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
