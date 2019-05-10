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
impl NondeterministicFiniteAutomata {
    pub fn printTable(&self){
        println!("{0: <10} | {1: <10} | {2: <10}", "from", "symbol", "to");
        for t in self.transition_function.iter() {
            /*
            let ((a, b), c) = t;
            let mut na = a.to_string().clone();
            let mut nc = c.to_string().clone();
            if na == self.start_state {
                na = "->".to_string() + &na;
            }
            if nc == self.start_state {
                nc = "->".to_string() + &nc;
            }
            if self.accept_states.contains(&na) {
                na = na + "*";
            }
            if self.accept_states.contains(&nc) {
                nc = nc + "*";
            }
            println!("{0: <10} | {1: <10} | {2: <10}", na, b, nc);
            */
        }
    }
    pub fn removeState(&mut self, state: &String) {
        /*
        let mut s = state.clone();
        if self.states.remove(&s) {
            let mut transitions = Vec::new();
            for (key, value) in self.transition_function.iter_mut() {
                let (key1, key2) = key;
                if key1 == state || key2 == state || value == state {
                    transitions.push(key.clone());
                }
            }
            for item in transitions {
                self.transition_function.remove(&item);
            }
        }
    */
    }
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
