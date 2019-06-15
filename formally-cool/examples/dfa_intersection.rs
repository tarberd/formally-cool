use formally_cool::regular_languages::*;
use std::collections::BTreeMap;

fn main() {
    let mut hash = BTreeMap::new();

    hash.insert((String::from("q0"), String::from("a")), String::from("q1"));
    hash.insert((String::from("q0"), String::from("b")), String::from("q0"));
    hash.insert((String::from("q1"), String::from("a")), String::from("q2"));
    hash.insert((String::from("q1"), String::from("b")), String::from("q1"));
    hash.insert((String::from("q2"), String::from("a")), String::from("q3"));
    hash.insert((String::from("q2"), String::from("b")), String::from("q2"));
    hash.insert((String::from("q3"), String::from("a")), String::from("q3"));
    hash.insert((String::from("q3"), String::from("b")), String::from("q3"));

    let automata = DeterministicFiniteAutomata {
        states: [
            "q0".to_string(),
            "q1".to_string(),
            "q2".to_string(),
            "q3".to_string(),
        ]
        .iter()
        .cloned()
        .collect(),
        alphabet: ["a".to_string(), "b".to_string()].iter().cloned().collect(),
        transition_function: hash,
        start_state: String::from("q0"),
        accept_states: [String::from("q2")].iter().cloned().collect(),
    };

    println!("Automata 1:");
    println!("{}", automata);

    let mut hash = BTreeMap::new();

    hash.insert((String::from("r0"), String::from("a")), String::from("r0"));
    hash.insert((String::from("r0"), String::from("b")), String::from("r1"));
    hash.insert((String::from("r1"), String::from("a")), String::from("r1"));
    hash.insert((String::from("r1"), String::from("b")), String::from("r2"));
    hash.insert((String::from("r2"), String::from("a")), String::from("r2"));
    hash.insert((String::from("r2"), String::from("b")), String::from("r2"));

    let automata2 = DeterministicFiniteAutomata {
        states: ["r0".to_string(), "r1".to_string(), "r2".to_string()]
            .iter()
            .cloned()
            .collect(),
        alphabet: ["a".to_string(), "b".to_string()].iter().cloned().collect(),
        transition_function: hash,
        start_state: String::from("r0"),
        accept_states: [String::from("r2")].iter().cloned().collect(),
    };

    println!("Automata 2:");
    println!("{}", automata2);

    println!("Intersection:");
    println!("{}", automata.intersection(&automata2));
}
