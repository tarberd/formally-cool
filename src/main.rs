use std::collections::HashMap;

fn main() {
    let accept_state = "q0";
    let input = "ababab";
    let mut hash = HashMap::new();

    hash.insert(("q0", "b"), "q1");
    hash.insert(("q0", "a"), "q0");
    hash.insert(("q1", "a"), "q0");
    hash.insert(("q1", "b"), "q1");

    let mut actual_state = "q0";

    for symbol in input.chars() {
        actual_state = hash[&(actual_state, symbol.to_string().as_str())];
    }

    println!("Entry: {} Finish State: {}", input, actual_state);
}
