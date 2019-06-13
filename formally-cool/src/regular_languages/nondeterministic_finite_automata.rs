use super::deterministic_finite_automata::{set_to_state, DeterministicFiniteAutomata};
use super::regular_grammar::RegularGrammar;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;

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

impl fmt::Display for NondeterministicFiniteAutomata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let decoration_spacing = 3;

        let table_spacing = self
            .transition_function
            .iter()
            .fold(String::from(""), |bigger, (_, set)| {
                if bigger.len() >= set_to_state(&set).len() {
                    bigger
                } else {
                    set_to_state(&set)
                }
            })
            .len()
            + 3;

        let result = write!(
            f,
            "{:d$}{:width$}",
            "",
            "g",
            d = decoration_spacing,
            width = table_spacing,
        );

        write!(f, "{:width$}", "&", width = table_spacing)?;
        for letter in &self.alphabet {
            write!(f, "{:width$}", letter, width = table_spacing)?;
        }
        write!(f, "\n")?;

        for state in &self.states {
            let mut decorations = String::from("");
            if self.accept_states.contains(state) {
                decorations += "*";
            }
            if *state == self.start_state {
                decorations += "->";
            }
            write!(
                f,
                "{:>d$}{:width$}",
                decorations,
                state,
                d = decoration_spacing,
                width = table_spacing,
            )?;

            write!(
                f,
                "{:width$}",
                match self
                    .transition_function
                    .get(&(state.clone(), String::from("&")))
                {
                    Some(state) => set_to_state(state),
                    None => String::from("-"),
                },
                width = table_spacing,
            )?;
            for letter in &self.alphabet {
                write!(
                    f,
                    "{:width$}",
                    match self
                        .transition_function
                        .get(&(state.clone(), letter.clone()))
                    {
                        Some(state) => set_to_state(state),
                        None => String::from("-"),
                    },
                    width = table_spacing,
                )?;
            }

            if state != self.states.iter().last().unwrap() {
                write!(f, "\n")?;
            }
        }

        result
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

impl From<&DeterministicFiniteAutomata> for NondeterministicFiniteAutomata {
    fn from(dfa: &DeterministicFiniteAutomata) -> Self {
        let states = dfa.states.clone();
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

impl NondeterministicFiniteAutomata {
    pub fn union(&self, other: &Self) -> Self {
        let mut states: BTreeSet<String> = self.states.union(&other.states).cloned().collect();
        let alphabet: BTreeSet<String> = self.alphabet.union(&other.alphabet).cloned().collect();
        let mut transition_function = BTreeMap::new();
        let accept_states = self
            .accept_states
            .union(&other.accept_states)
            .cloned()
            .collect();

        let mut start_state = String::from("q");

        for i in 0.. {
            start_state = String::from("q") + &i.to_string();
            if !states.contains(&start_state) {
                states.insert(start_state.clone());
                break;
            }
        }

        transition_function.insert(
            (start_state.clone(), String::from("&")),
            [self.start_state.clone(), other.start_state.clone()]
                .iter()
                .cloned()
                .collect(),
        );

        for state in &states {
            for letter in &alphabet {
                let self_set = match self
                    .transition_function
                    .get(&(state.clone(), letter.clone()))
                {
                    Some(set) => set.clone(),
                    None => BTreeSet::new(),
                };

                let other_set = match other
                    .transition_function
                    .get(&(state.clone(), letter.clone()))
                {
                    Some(set) => set.clone(),
                    None => BTreeSet::new(),
                };

                transition_function.insert(
                    (state.clone(), letter.clone()),
                    self_set.union(&other_set).cloned().collect(),
                );
            }
        }

        NondeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: transition_function,
            start_state: start_state,
            accept_states: accept_states,
        }
    }
}
