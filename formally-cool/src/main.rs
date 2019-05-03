use std::collections::HashMap;

use std::fs::File;
use std::io::{BufReader, BufWriter};

use formally_cool::regular_languages::*;

fn main() {
    let input = String::from("abababa");
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

    let result = automata.compute(input.as_str());

    println!("Result {}", result);

    println!("{:#?}", automata);

    let regular_grammar = RegularGrammar::from(&automata);

    println!("{:#?}", regular_grammar);

    let automata = NondeterministicFiniteAutomata::from(&regular_grammar);

    println!("{:#?}", automata);

    let serialized = serde_yaml::to_string(&automata).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: NondeterministicFiniteAutomata = serde_yaml::from_str(&serialized).unwrap();
    println!("deserialized = {:#?}", deserialized);

    match File::create("automata.yaml") {
        Ok(file) => {
            let writer = BufWriter::new(file);

            serde_yaml::to_writer(writer, &automata).unwrap();
        }
        Err(e) => println!("error : {:?}", e),
    }

    match File::open("automata.yaml") {
        Ok(file) => {
            let reader = BufReader::new(file);

            let automata: NondeterministicFiniteAutomata = serde_yaml::from_reader(reader).unwrap();

            println!("from file = {:#?}", automata);
        }
        Err(e) => println!("error : {:?}", e),
    }

    let automata = DeterministicFiniteAutomata::from(&automata);

    println!("{:#?}", automata);
}
