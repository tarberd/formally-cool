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
    pub start_symble: String,
    pub productions: HashMap<String, Vec<String>>,
}

pub fn state_to_symble(state: &String) -> String {
    String::from("<") + state + ">"
}

pub fn make_regular_grammar(automata: &Automata) -> RegularGrammar {
    let mut productions = HashMap::new();

    for ((state, _), _) in &automata.transition_function {
        productions.insert(state_to_symble(state), vec![]);
    }

    for ((state, entry_symble), next_state) in &automata.transition_function {
        match productions.get_mut(&state_to_symble(state)) {
            Some(x) => x.push(entry_symble.clone() + &state_to_symble(next_state)),
            None => (),
        }

        if automata
            .accept_states
            .contains(&state_to_symble(next_state))
        {
            match productions.get_mut(&state_to_symble(state)) {
                Some(x) => x.push(entry_symble.clone()),
                None => (),
            }
        }
    }

    RegularGrammar {
        start_symble: state_to_symble(&automata.start_state),
        productions: productions,
    }
}
