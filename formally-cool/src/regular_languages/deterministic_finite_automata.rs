use std::collections::HashMap;
use std::collections::HashSet;

use super::nondeterministic_finite_automata::NondeterministicFiniteAutomata;

#[derive(Debug, Clone)]
pub struct DeterministicFiniteAutomata {
    pub start_state: String,
    pub transition_function: HashMap<(String, String), String>,
    pub accept_states: HashSet<String>,
}

impl DeterministicFiniteAutomata {
    pub fn compute(&self, input: &str) -> bool {
        let mut actual_state = self.start_state.clone();
        for symbol in input.chars() {
            actual_state = self.transition_function[&(actual_state, symbol.to_string())].clone();
        }
        self.accept_states.contains(&actual_state)
    }
}

impl From<&NondeterministicFiniteAutomata> for DeterministicFiniteAutomata {
    fn from(automata: &NondeterministicFiniteAutomata) -> Self {
        DeterministicFiniteAutomata {
            start_state: String::new(),
            transition_function: HashMap::new(),
            accept_states: HashSet::new(),
        }
    }
}
