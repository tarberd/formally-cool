use formally_cool::regular_languages::*;
use std::collections::BTreeMap;

fn main() {
    let variables = [
        "<S`>".to_string(),
        "<S>".to_string(),
        "<A>".to_string(),
        "<C>".to_string(),
        "<D>".to_string(),
        "<E>".to_string(),
    ]
    .iter()
    .cloned()
    .collect();

    let terminals = ["a".to_string(), "b".to_string(), "c".to_string()]
        .iter()
        .cloned()
        .collect();

    let mut rules = BTreeMap::new();

    rules.insert(
        "<S`>".to_string(),
        [
            "a<A>".to_string(),
            "c<C>".to_string(),
            "b<A>".to_string(),
            "b<C>".to_string(),
            "&".to_string(),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    rules.insert(
        "<S>".to_string(),
        [
            "a<A>".to_string(),
            "c<C>".to_string(),
            "b<A>".to_string(),
            "b<C>".to_string(),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    rules.insert(
        "<A>".to_string(),
        [
            "b<S>".to_string(),
            "c<D>".to_string(),
            "b".to_string(),
            "c".to_string(),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    rules.insert(
        "<C>".to_string(),
        [
            "b<S>".to_string(),
            "a<E>".to_string(),
            "b".to_string(),
            "a".to_string(),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    rules.insert(
        "<D>".to_string(),
        ["a<A>".to_string(), "b<A>".to_string(), "b<C>".to_string()]
            .iter()
            .cloned()
            .collect(),
    );

    rules.insert(
        "<E>".to_string(),
        ["c<C>".to_string(), "b<C>".to_string(), "b<A>".to_string()]
            .iter()
            .cloned()
            .collect(),
    );

    let start_variable = String::from("<S`>");

    let grammar = Grammar {
        variables: variables,
        terminals: terminals,
        rules: rules,
        start_variable: start_variable,
    };

    println!("{:#?}", grammar);

    let automata = NondeterministicFiniteAutomata::from(&grammar);

    println!("Start state : {}", automata.start_state);

    let automata = DeterministicFiniteAutomata::from(&automata);

    println!("Start state : {}", automata.start_state);

    let minimized = automata.minimize();

    println!("{:#?}", minimized);
}
