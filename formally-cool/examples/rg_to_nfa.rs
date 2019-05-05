use std::collections::HashMap;

use formally_cool::regular_languages::*;

fn main() {
    let mut hash = HashMap::new();

    hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
    hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
    hash.insert((String::from("q1"), String::from("a")), String::from("q0"));
    hash.insert((String::from("q1"), String::from("b")), String::from("q1"));

    let automata = DeterministicFiniteAutomata {
        start_state: String::from("q0"),
        transition_function: hash,
        accept_states: [String::from("q0")].iter().cloned().collect(),
    };

    let regular_grammar = RegularGrammar::from(&automata);

    println!("{:#?}", regular_grammar);

    let automata = NondeterministicFiniteAutomata::from(&regular_grammar);

    println!("{:#?}", automata);
}
