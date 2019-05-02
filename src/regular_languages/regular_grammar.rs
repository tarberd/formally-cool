use std::collections::HashMap;
use std::collections::HashSet;

use super::deterministic_finite_automata::*;

fn state_to_symbol(state: &String) -> String {
    String::from("<") + state + ">"
}

#[derive(Debug, Clone)]
pub struct RegularGrammar {
    pub start_symbol: String,
    pub productions: HashMap<String, HashSet<String>>,
}

impl From<&DeterministicFiniteAutomata> for RegularGrammar {
    fn from(automata: &DeterministicFiniteAutomata) -> Self {
        let mut productions = HashMap::new();

        for ((state, _), _) in &automata.transition_function {
            productions.insert(state_to_symbol(state), HashSet::new());
        }

        for ((state, entry_symbol), next_state) in &automata.transition_function {
            match productions.get_mut(&state_to_symbol(state)) {
                Some(set) => set.insert(entry_symbol.clone() + &state_to_symbol(next_state)),
                None => false,
            };

            if automata.accept_states.contains(&next_state.clone()) {
                match productions.get_mut(&state_to_symbol(state)) {
                    Some(set) => set.insert(entry_symbol.clone()),
                    None => false,
                };
            }
        }

        RegularGrammar {
            start_symbol: state_to_symbol(&automata.start_state),
            productions: productions,
        }
    }
}
