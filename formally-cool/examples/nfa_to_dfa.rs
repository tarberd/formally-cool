use formally_cool::regular_languages::*;
use std::collections::BTreeMap;

fn main() {
    let automata = NondeterministicFiniteAutomata {
        states: ["p".to_string(), "q".to_string(), "r".to_string()]
            .iter()
            .cloned()
            .collect(),
        alphabet: ["a".to_string(), "b".to_string(), "c".to_string()]
            .iter()
            .cloned()
            .collect(),
        start_state: String::from("p"),
        transition_function: BTreeMap::new(),
        accept_states: ["r".to_string()].iter().cloned().collect(),
    };

    println!("{:#?}", automata);

    let automata = DeterministicFiniteAutomata::from(&automata);

    println!("{:#?}", automata);
}
