use formally_cool::regular_languages::*;
use std::collections::BTreeMap;

fn main() {
    let mut hash = BTreeMap::new();

    hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
    hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
    hash.insert((String::from("q1"), String::from("a")), String::from("q0"));
    hash.insert((String::from("q1"), String::from("b")), String::from("q1"));

    let automata = DeterministicFiniteAutomata {
        states: ["q0".to_string(), "q1".to_string()]
            .iter()
            .cloned()
            .collect(),
        alphabet: ["a".to_string(), "b".to_string()].iter().cloned().collect(),
        transition_function: hash,
        start_state: String::from("q0"),
        accept_states: [String::from("q0")].iter().cloned().collect(),
    };

    println!("Automata 1:");
    println!("{}", automata);

    let mut hash = BTreeMap::new();

    hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
    hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
    hash.insert((String::from("q1"), String::from("a")), String::from("q0"));
    hash.insert((String::from("q1"), String::from("b")), String::from("q1"));

    let automata2 = DeterministicFiniteAutomata {
        states: ["q0".to_string(), "q1".to_string()]
            .iter()
            .cloned()
            .collect(),
        alphabet: ["a".to_string(), "b".to_string()].iter().cloned().collect(),
        transition_function: hash,
        start_state: String::from("q1"),
        accept_states: [String::from("q1")].iter().cloned().collect(),
    };

    println!("Automata 2:");
    println!("{}", automata2);

    println!("Intersection:");
    println!("{}", automata.intersection(&automata2));
}
