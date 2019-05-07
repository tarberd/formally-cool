use std::collections::BTreeMap;

use std::fs::File;
use std::io::{BufReader, BufWriter};

use formally_cool::regular_languages::*;

fn main() {
    let input = String::from("abababa");
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

    let result = automata.compute(input.as_str());

    println!("Result {}", result);

    println!("{:#?}", automata);

    let regular_grammar = RegularGrammar::from(&automata);

    println!("{:#?}", regular_grammar);

    let automata = NondeterministicFiniteAutomata::from(&regular_grammar);

    println!("{:#?}", automata);

    let automata = DeterministicFiniteAutomata::from(&automata);

    println!("{:#?}", automata);
}
