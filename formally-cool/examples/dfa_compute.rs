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

    println!("DFA for w = z.a");
    println!("{}", automata);

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
