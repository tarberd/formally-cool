use formally_cool::regular_languages::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let mut transition_function: BTreeMap<(String, String), BTreeSet<String>> = BTreeMap::new();

    transition_function.insert(
        ("q1".to_string(), "0".to_string()),
        ["q1".to_string()].iter().cloned().collect(),
    );
    transition_function.insert(
        ("q1".to_string(), "1".to_string()),
        ["q1".to_string(), "q2".to_string()]
            .iter()
            .cloned()
            .collect(),
    );
    transition_function.insert(
        ("q2".to_string(), "0".to_string()),
        ["q3".to_string()].iter().cloned().collect(),
    );
    transition_function.insert(
        ("q2".to_string(), "1".to_string()),
        ["q3".to_string()].iter().cloned().collect(),
    );
    transition_function.insert(
        ("q3".to_string(), "0".to_string()),
        ["q4".to_string()].iter().cloned().collect(),
    );
    transition_function.insert(
        ("q3".to_string(), "1".to_string()),
        ["q4".to_string()].iter().cloned().collect(),
    );

    let automata = NondeterministicFiniteAutomata {
        states: [
            "q1".to_string(),
            "q2".to_string(),
            "q3".to_string(),
            "q4".to_string(),
        ]
        .iter()
        .cloned()
        .collect(),
        alphabet: ["0".to_string(), "1".to_string()].iter().cloned().collect(),
        start_state: String::from("q1"),
        transition_function: transition_function,
        accept_states: ["q4".to_string()].iter().cloned().collect(),
    };

    println!("{}", automata);

    let automata = DeterministicFiniteAutomata::from(&automata);

    println!("{}", automata);
}
