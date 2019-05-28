use super::nondeterministic_finite_automata::NondeterministicFiniteAutomata;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

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

    pub fn remove_unreachable_states(&self) -> Self {
        let mut states;
        let alphabet = self.alphabet.clone();
        let mut transition_function = BTreeMap::new();
        let start_state = self.start_state.clone();
        let mut accept_states = BTreeSet::new();

        {
            let mut before = BTreeSet::new();

            before.insert(self.start_state.clone());

            let mut after = BTreeSet::new();

            let mut stop = false;
            while !stop {
                for state in &before {
                    after.insert(state.clone());

                    for letter in &alphabet {
                        match self
                            .transition_function
                            .get(&(state.clone(), letter.clone()))
                        {
                            Some(state) => {
                                after.insert(state.clone());
                            }
                            None => (),
                        }
                    }
                }

                if after.is_subset(&before) && after.is_superset(&before) {
                    stop = true;
                } else {
                    before = after.clone();
                }
            }

            states = before;
        }

        for ((state, letter), state_out) in &self.transition_function {
            if states.contains(state) && states.contains(state_out) {
                transition_function.insert((state.clone(), letter.clone()), state_out.clone());
            }
        }

        for state in &self.accept_states {
            if states.contains(state) {
                accept_states.insert(state.clone());
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

    pub fn remove_non_productive_states(&self) -> Self {
        let mut states;
        let alphabet = self.alphabet.clone();
        let mut transition_function = BTreeMap::new();
        let start_state = self.start_state.clone();
        let accept_states = self.accept_states.clone();

        {
            let mut before = accept_states.clone();

            let mut after = BTreeSet::new();

            let mut stop = false;
            while !stop {
                for ((state, _), state_out) in &self.transition_function {
                    if before.contains(state_out) {
                        after.insert(state.clone());
                    }
                }

                if after.is_subset(&before) && after.is_superset(&before) {
                    stop = true;
                } else {
                    before = after.clone();
                }
            }

            states = before;
        }

        for ((state, letter), state_out) in &self.transition_function {
            if states.contains(state) && states.contains(state_out) {
                transition_function.insert((state.clone(), letter.clone()), state_out.clone());
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

fn powerset<T: Clone>(slice: &[T]) -> Vec<Vec<T>> {
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
        let mut start_state;
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

        let mut epsilon_closure = BTreeMap::new();

        {
            for state in &automata.states {
                let mut before = BTreeSet::new();

                before.insert(state.clone());

                let mut after = BTreeSet::new();

                let mut stop = false;
                while !stop {
                    after = BTreeSet::new();

                    for state in &before {
                        after.insert(state.clone());

                        match automata
                            .transition_function
                            .get(&(state.clone(), "&".to_string()))
                        {
                            Some(set) => {
                                for state in set {
                                    after.insert(state.clone());
                                }
                            }
                            None => (),
                        }
                    }

                    if after.is_subset(&before) && after.is_superset(&before) {
                        stop = true;
                    } else {
                        before = after.clone();
                    }
                }

                epsilon_closure.insert(state.clone(), after);
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
                            for state in set {
                                let epsilon_closure_for_state = epsilon_closure.get(state).unwrap();

                                for state in epsilon_closure_for_state {
                                    output_state_set.insert(state.clone());
                                }
                            }
                        }
                        None => (),
                    }
                }

                let mut output_state_name = String::new();
                for state_name in output_state_set {
                    output_state_name = output_state_name + state_name.as_str() + ".";
                }

                transition_function.insert((state.clone(), letter.clone()), output_state_name);
            }
        }

        {
            let start_state_closure = epsilon_closure.get(&automata.start_state).unwrap();

            let mut output_state_name = String::new();
            for state_name in start_state_closure {
                output_state_name = output_state_name + state_name.as_str() + "_";
            }

            start_state = output_state_name;
        }

        let mut underscore_states = BTreeSet::new();
        let mut underscore_transition_function = BTreeMap::new();
        let mut underscore_accept_states = BTreeSet::new();

        for state in states {
            let underscore_state = state.as_str().replace(".", "_");
            underscore_states.insert(underscore_state);
        }

        for ((state, letter), out_state) in transition_function {
            let underscore_state = state.as_str().replace(".", "_");
            let underscore_out_state = out_state.as_str().replace(".", "_");

            underscore_transition_function
                .insert((underscore_state, letter.clone()), underscore_out_state);
        }

        for state in accept_states {
            let underscore_state = state.as_str().replace(".", "_");

            underscore_accept_states.insert(underscore_state);
        }

        DeterministicFiniteAutomata {
            states: underscore_states,
            alphabet: alphabet,
            transition_function: underscore_transition_function,
            start_state: start_state,
            accept_states: underscore_accept_states,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_unreachable_states() {
        let mut hash = BTreeMap::new();

        hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
        hash.insert((String::from("q1"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q1"), String::from("b")), String::from("q1"));
        hash.insert((String::from("q2"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q2"), String::from("b")), String::from("q1"));
        hash.insert((String::from("q3"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q3"), String::from("b")), String::from("q1"));

        let states = [
            "q0".to_string(),
            "q1".to_string(),
            "q2".to_string(),
            "q3".to_string(),
        ]
        .iter()
        .cloned()
        .collect();

        let alphabet = ["a".to_string(), "b".to_string()].iter().cloned().collect();

        let accept_states = [String::from("q0"), String::from("q3")]
            .iter()
            .cloned()
            .collect();

        let automata = DeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: hash,
            start_state: String::from("q0"),
            accept_states: accept_states,
        };

        let automata = automata.remove_unreachable_states();

        let result_states: BTreeSet<_> = ["q0".to_string(), "q1".to_string()]
            .iter()
            .cloned()
            .collect();

        assert_eq!(automata.states, result_states);

        let result_alphabet = ["a".to_string(), "b".to_string()].iter().cloned().collect();

        assert_eq!(automata.alphabet, result_alphabet);

        let mut result_hash = BTreeMap::new();

        result_hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
        result_hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
        result_hash.insert((String::from("q1"), String::from("a")), String::from("q0"));
        result_hash.insert((String::from("q1"), String::from("b")), String::from("q1"));

        assert_eq!(automata.transition_function, result_hash);

        let result_start_state = String::from("q0");

        assert_eq!(automata.start_state, result_start_state);

        let accept_states_result = [String::from("q0")].iter().cloned().collect();

        assert_eq!(automata.accept_states, accept_states_result);
    }

    #[test]
    fn remove_non_productive_states() {
        let mut hash = BTreeMap::new();

        hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
        hash.insert((String::from("q1"), String::from("a")), String::from("q2"));
        hash.insert((String::from("q1"), String::from("b")), String::from("q0"));
        hash.insert((String::from("q2"), String::from("a")), String::from("q3"));
        hash.insert((String::from("q2"), String::from("b")), String::from("q4"));
        hash.insert((String::from("q3"), String::from("a")), String::from("q4"));
        hash.insert((String::from("q3"), String::from("b")), String::from("q3"));
        hash.insert((String::from("q4"), String::from("a")), String::from("q3"));
        hash.insert((String::from("q4"), String::from("b")), String::from("q4"));

        let states = [
            "q0".to_string(),
            "q1".to_string(),
            "q2".to_string(),
            "q3".to_string(),
            "q4".to_string(),
        ]
        .iter()
        .cloned()
        .collect();

        let alphabet = ["a".to_string(), "b".to_string()].iter().cloned().collect();

        let accept_states = [String::from("q0")].iter().cloned().collect();

        let automata = DeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: hash,
            start_state: String::from("q0"),
            accept_states: accept_states,
        };

        let automata = automata.remove_non_productive_states();

        let result_states: BTreeSet<_> = ["q0".to_string(), "q1".to_string()]
            .iter()
            .cloned()
            .collect();

        assert_eq!(automata.states, result_states);

        let result_alphabet = ["a".to_string(), "b".to_string()].iter().cloned().collect();

        assert_eq!(automata.alphabet, result_alphabet);

        let mut result_hash = BTreeMap::new();

        result_hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
        result_hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
        result_hash.insert((String::from("q1"), String::from("b")), String::from("q0"));

        assert_eq!(automata.transition_function, result_hash);

        let result_start_state = String::from("q0");

        assert_eq!(automata.start_state, result_start_state);

        let accept_states_result = [String::from("q0")].iter().cloned().collect();

        assert_eq!(automata.accept_states, accept_states_result);
    }
}
