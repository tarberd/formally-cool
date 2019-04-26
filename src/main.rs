use formally_cool::make_regular_grammar;
use formally_cool::Automata;
use std::collections::HashMap;

fn main() {
    let input = String::from("abababa");
    let mut hash = HashMap::new();

    hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
    hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
    hash.insert((String::from("q1"), String::from("a")), String::from("q0"));
    hash.insert((String::from("q1"), String::from("b")), String::from("q1"));

    let automata = Automata {
        start_state: String::from("q0"),
        transition_function: hash,
        accept_states: [String::from("q0")].iter().cloned().collect(),
    };

    let result = automata.compute(input.as_str());

    println!("Result {}", result);

    println!("{:?}", automata);

    let regular_grammar = make_regular_grammar(&automata);

    println!("{:?}", regular_grammar);
}
