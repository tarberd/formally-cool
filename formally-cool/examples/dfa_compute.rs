use formally_cool::regular_languages::*;
use std::collections::HashMap;

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

    println!("DFA for w = z.a");
    println!("{:#?}", automata);

    let input = String::from("babba");

    let result = automata.compute(input.as_str());

    println!("Result for input {}: {}", input, result);

    let input = String::from("abab");

    let result = automata.compute(input.as_str());

    println!("Result for input {}: {}", input, result);

    let input = String::from("aaa");

    let result = automata.compute(input.as_str());

    println!("Result for input {}: {}", input, result);

    let input = String::from("bbb");

    let result = automata.compute(input.as_str());

    println!("Result for input {}: {}", input, result);
}
