use super::deterministic_finite_automata::DeterministicFiniteAutomata;
use super::regular_grammar::RegularGrammar;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn symbol_to_state(symbol: &String) -> String {
    symbol[1..(symbol.len() - 1)].to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NondeterministicFiniteAutomata {
    pub states: BTreeSet<String>,
    pub alphabet: BTreeSet<String>,
    pub start_state: String,
    pub transition_function: BTreeMap<(String, String), BTreeSet<String>>,
    pub accept_states: BTreeSet<String>,
}

impl From<&RegularGrammar> for NondeterministicFiniteAutomata {
    fn from(regular_grammar: &RegularGrammar) -> Self {
        let mut states = BTreeSet::new();
        let mut alphabet = BTreeSet::new();
        let mut transition_function = BTreeMap::new();
        let mut accept_states = BTreeSet::new();

        for variable in &regular_grammar.variables {
            states.insert(symbol_to_state(variable));
        }

        for terminal in &regular_grammar.terminals {
            alphabet.insert(terminal.clone());
        }

        let final_state = String::from("accept");

        states.insert(final_state.clone());
        accept_states.insert(final_state.clone());

        for state in &states {
            for letter in &alphabet {
                transition_function.insert((state.clone(), letter.clone()), BTreeSet::new());
            }
        }

        for (symbol, set) in &regular_grammar.rules {
            for derivation in set {
                let key = (symbol_to_state(symbol), derivation[0..1].to_string());

                if !transition_function.contains_key(&key) {
                    transition_function.insert(key.clone(), BTreeSet::new());
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
            states: states,
            alphabet: alphabet,
            transition_function: transition_function,
            start_state: symbol_to_state(&regular_grammar.start_variable),
            accept_states: accept_states,
        }
    }
}

impl From<&DeterministicFiniteAutomata> for NondeterministicFiniteAutomata {
    fn from(dfa: &DeterministicFiniteAutomata) -> Self {
        let states = dfa.alphabet.clone();
        let alphabet = dfa.alphabet.clone();
        let mut transition_function = BTreeMap::new();
        let accept_states = dfa.accept_states.clone();

        for state in &states {
            for letter in &alphabet {
                match dfa
                    .transition_function
                    .get(&(state.clone(), letter.clone()))
                {
                    Some(out_state) => {
                        let mut out_set = BTreeSet::new();
                        out_set.insert(out_state.clone());
                        transition_function.insert((state.clone(), letter.clone()), out_set);
                    }
                    None => (),
                }
            }
        }

        NondeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: transition_function,
            start_state: dfa.start_state.clone(),
            accept_states: accept_states,
        }
    }
}
