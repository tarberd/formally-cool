use super::nondeterministic_finite_automata::NondeterministicFiniteAutomata;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeterministicFiniteAutomata {
    pub states: BTreeSet<String>,
    pub alphabet: BTreeSet<String>,
    pub transition_function: BTreeMap<(String, String), String>,
    pub start_state: String,
    pub accept_states: BTreeSet<String>,
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

pub fn powerset<T: Clone>(slice: &[T]) -> Vec<Vec<T>> {
    let mut v: Vec<Vec<T>> = Vec::new();

    for mask in 0..(1 << slice.len()) {
        let mut ss: Vec<T> = vec![];
        let mut bitset = mask;
        while bitset > 0 {
            // isolate the rightmost bit to select one item
            let rightmost: u64 = bitset & !(bitset - 1);
            // turn the isolated bit into an array index
            let idx = rightmost.trailing_zeros();
            let item = (*slice.get(idx as usize).unwrap()).clone();
            ss.push(item);
            // zero the trailing bit
            bitset &= bitset - 1;
        }
        v.push(ss);
    }
    v
}

impl From<&NondeterministicFiniteAutomata> for DeterministicFiniteAutomata {
    fn from(automata: &NondeterministicFiniteAutomata) -> Self {
        let mut states = BTreeSet::new();
        let mut alphabet = BTreeSet::new();
        let mut transition_function = BTreeMap::new();
        let start_state = automata.start_state.clone() + ".";
        let mut accept_states = BTreeSet::new();

        for letter in &automata.alphabet {
            alphabet.insert(letter.clone());
        }

        {
            let mut state_set_vec = vec![];

            for state in &automata.states {
                state_set_vec.push(state.clone());
            }

            let power_state_set_vec = powerset(&state_set_vec);

            for vec in &power_state_set_vec {
                let mut state_name = String::new();
                for state in vec {
                    state_name = state_name + state + ".";
                }
                states.insert(state_name);
            }
        }

        for state in &states {
            for letter in &alphabet {
                let mut output_state_set = BTreeSet::new();

                let split_state_names: Vec<&str> = state.split_terminator('.').collect();

                for state_name in split_state_names {
                    if automata.accept_states.contains(state_name) {
                        accept_states.insert(state.clone());
                    }
                    match automata
                        .transition_function
                        .get(&(state_name.to_owned(), letter.clone()))
                    {
                        Some(set) => {
                            for s in set {
                                output_state_set.insert(s.clone());
                            }
                        }
                        None => (),
                    }
                }

                println!("{:?}", output_state_set);

                let mut output_state_name = String::new();
                for state_name in output_state_set {
                    output_state_name = output_state_name + state_name.as_str() + ".";
                }

                println!("{:?}", output_state_name);

                transition_function.insert((state.clone(), letter.clone()), output_state_name);
            }
        }

        DeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: transition_function,
            start_state: start_state,
            accept_states: accept_states,
        }
    }
}
