use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Automata {
    start_state: String,
    transition_function: HashMap<(String, String), String>,
    accept_states: HashSet<String>,
}

impl Automata {
    fn compute(&self, input: &str) -> bool {
        let mut actual_state = self.start_state.clone();
        for symbol in input.chars() {
            actual_state = self.transition_function[&(actual_state, symbol.to_string())].clone();
        }
        self.accept_states.contains(&actual_state)
    }
}

#[derive(Debug, Clone)]
struct RegularGrammar {
    start_symble: String,
    productions: HashMap<String, Vec<String>>,
}

fn state_to_symble(state: &String) -> String {
    String::from("<") + state + ">"
}

fn make_regular_grammar(automata: &Automata) -> RegularGrammar {
    let mut productions = HashMap::new();

    for ((state, _), _) in &automata.transition_function {
        productions.insert(state_to_symble(state), vec![]);
    }

    for ((state, entry_symble), next_state) in &automata.transition_function {
        match productions.get_mut(&state_to_symble(state)) {
            Some(x) => x.push(entry_symble.clone() + &state_to_symble(next_state)),
            None => (),
        }

        if automata
            .accept_states
            .contains(&state_to_symble(next_state))
        {
            match productions.get_mut(&state_to_symble(state)) {
                Some(x) => x.push(entry_symble.clone()),
                None => (),
            }
        }
    }

    RegularGrammar {
        start_symble: state_to_symble(&automata.start_state),
        productions: productions,
    }
}

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
