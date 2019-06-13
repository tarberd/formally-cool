use super::nondeterministic_finite_automata::NondeterministicFiniteAutomata;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeterministicFiniteAutomata {
    pub states: BTreeSet<String>,
    pub alphabet: BTreeSet<String>,
    pub transition_function: BTreeMap<(String, String), String>,
    pub start_state: String,
    pub accept_states: BTreeSet<String>,
}

impl fmt::Display for DeterministicFiniteAutomata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let decoration_spacing = 3;

        let table_spacing = self
            .states
            .iter()
            .cloned()
            .fold(String::from(""), |bigger, next| {
                if bigger.len() < next.len() {
                    next
                } else {
                    bigger
                }
            })
            .len()
            + 3;

        let result = write!(
            f,
            "{:d$}{:width$}",
            "",
            "g",
            d = decoration_spacing,
            width = table_spacing,
        );

        for letter in &self.alphabet {
            write!(f, "{:width$}", letter, width = table_spacing)?;
        }
        write!(f, "\n")?;

        for state in &self.states {
            let mut decorations = String::from("");
            if self.accept_states.contains(state) {
                decorations += "*";
            }
            if *state == self.start_state {
                decorations += "->";
            }
            write!(
                f,
                "{:>d$}{:width$}",
                decorations,
                state,
                d = decoration_spacing,
                width = table_spacing,
            )?;

            for letter in &self.alphabet {
                write!(
                    f,
                    "{:width$}",
                    match self
                        .transition_function
                        .get(&(state.clone(), letter.clone()))
                    {
                        Some(state) => state.clone(),
                        None => String::from("-"),
                    },
                    width = table_spacing,
                )?;
            }

            if state != self.states.iter().last().unwrap() {
                write!(f, "\n")?;
            }
        }

        result
    }
}

pub fn state_to_set(state: &String) -> BTreeSet<String> {
    let mut set = BTreeSet::new();

    if state == "" {
        set.insert(state.clone());
    } else if state.chars().next().unwrap() != '(' {
        set.insert(state.as_str()[0..state.len()].to_owned());
    } else if state.len() == 2 {
        set.insert("".to_owned());
    } else {
        let naked_state = state.get(1..(state.len() - 1)).unwrap();

        let mut bracket_count = 0;

        let mut start = 0;

        for index in 0..naked_state.len() {
            let letter = naked_state.get(index..(index + 1)).unwrap();

            if bracket_count == 0 {
                if letter == "," {
                    let end = index;
                    set.insert(naked_state[start..end].to_owned());
                    start = index + 2;
                }
                if index == naked_state.len() - 1 {
                    let end = index + 1;
                    set.insert(naked_state[start..end].to_owned());
                }
                if letter == "(" {
                    bracket_count += 1;
                    start = index;
                }
            } else {
                if letter == "(" {
                    bracket_count += 1;
                }
                if letter == ")" {
                    bracket_count -= 1;

                    if bracket_count == 0 {
                        let end = index + 1;
                        set.insert(naked_state[start..end].to_owned());
                    }
                }
            }
        }
    }

    set
}

pub fn set_to_state(set: &BTreeSet<String>) -> String {
    let mut concatenated_state = String::from("(");

    for state in set {
        if state != set.iter().last().unwrap() {
            concatenated_state = concatenated_state + state + ", ";
        } else {
            concatenated_state = concatenated_state + state;
        }
    }

    concatenated_state += ")";

    concatenated_state
}

impl DeterministicFiniteAutomata {
    pub fn compute(&self, input: &str) -> bool {
        let mut actual_state = self.start_state.clone();
        for symbol in input.chars() {
            actual_state = self.transition_function[&(actual_state, symbol.to_string())].clone();
        }
        self.accept_states.contains(&actual_state)
    }

    pub fn remove_unreachable_states(&self) -> Self {
        let mut states;
        let alphabet = self.alphabet.clone();
        let mut transition_function = BTreeMap::new();
        let start_state = self.start_state.clone();
        let mut accept_states = BTreeSet::new();

        {
            let mut before = BTreeSet::new();

            before.insert(self.start_state.clone());

            let mut after = BTreeSet::new();

            let mut stop = false;
            while !stop {
                for state in &before {
                    after.insert(state.clone());

                    for letter in &alphabet {
                        match self
                            .transition_function
                            .get(&(state.clone(), letter.clone()))
                        {
                            Some(state) => {
                                after.insert(state.clone());
                            }
                            None => (),
                        }
                    }
                }

                if after.is_subset(&before) && after.is_superset(&before) {
                    stop = true;
                } else {
                    before = after.clone();
                }
            }

            states = before;
        }

        for ((state, letter), state_out) in &self.transition_function {
            if states.contains(state) && states.contains(state_out) {
                transition_function.insert((state.clone(), letter.clone()), state_out.clone());
            }
        }

        for state in &self.accept_states {
            if states.contains(state) {
                accept_states.insert(state.clone());
            }
        }

        DeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: transition_function,
            start_state: start_state,
            accept_states: accept_states,
        }
    }

    pub fn remove_non_productive_states(&self) -> Self {
        let mut states;
        let alphabet = self.alphabet.clone();
        let mut transition_function = BTreeMap::new();
        let start_state = self.start_state.clone();
        let accept_states = self.accept_states.clone();

        {
            let mut before = accept_states.clone();

            let mut after = BTreeSet::new();

            let mut stop = false;
            while !stop {
                for ((state, _), state_out) in &self.transition_function {
                    if before.contains(state_out) {
                        after.insert(state.clone());
                    }
                }

                if after.is_subset(&before) && after.is_superset(&before) {
                    stop = true;
                } else {
                    before = after.clone();
                }
            }

            states = before;
        }

        for ((state, letter), state_out) in &self.transition_function {
            if states.contains(state) && states.contains(state_out) {
                transition_function.insert((state.clone(), letter.clone()), state_out.clone());
            }
        }

        DeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: transition_function,
            start_state: start_state,
            accept_states: accept_states,
        }
    }

    pub fn remove_equivalent_states(&self) -> Self {
        let mut states = BTreeSet::new();
        let alphabet = self.alphabet.clone();
        let mut transition_function = BTreeMap::new();
        let mut start_state = String::new();
        let mut accept_states = BTreeSet::new();

        // P := {F, Q \ F};
        let mut equivalence_classes = BTreeSet::new();

        let accept_states_equivalence = self.accept_states.clone();
        let non_accept_states_equivalence = self
            .states
            .difference(&self.accept_states)
            .cloned()
            .collect();

        equivalence_classes.insert(accept_states_equivalence.clone());
        equivalence_classes.insert(non_accept_states_equivalence);

        // W := {F};
        let mut equivalence_classes_aux = BTreeSet::new();

        equivalence_classes_aux.insert(accept_states_equivalence);

        // while (W is not empty) do
        while !equivalence_classes_aux.is_empty() {
            // choose and remove a set A from W
            let current_set = equivalence_classes_aux.iter().cloned().last().unwrap();
            equivalence_classes_aux.remove(&current_set);

            // for each c in Σ do
            for letter in &alphabet {
                // let X be the set of states for which a transition on c leads to a state in A
                let mut auxiliar_set = BTreeSet::new();

                for ((state, input), out_state) in &self.transition_function {
                    if input == letter {
                        if current_set.contains(out_state) {
                            auxiliar_set.insert(state.clone());
                        }
                    }
                }

                // for each set Y in P for which X ∩ Y is nonempty and Y \ X is nonempty do
                let mut equivalence_classes_vector: Vec<_> =
                    equivalence_classes.iter().cloned().collect();

                let mut i = 0;
                while i < equivalence_classes_vector.len() {
                    let intersection: BTreeSet<String> = auxiliar_set
                        .intersection(&equivalence_classes_vector[i])
                        .cloned()
                        .collect();

                    let difference: BTreeSet<String> = equivalence_classes_vector[i]
                        .difference(&auxiliar_set)
                        .cloned()
                        .collect();

                    if !intersection.is_empty() && !difference.is_empty() {
                        // replace Y in P by the two sets X ∩ Y and Y \ X
                        let equivalence_class = equivalence_classes_vector.remove(i);
                        equivalence_classes_vector.push(intersection.clone());
                        equivalence_classes_vector.push(difference.clone());

                        // if Y is in W
                        //     replace Y in W by the same two sets
                        // else
                        //     if |X ∩ Y| <= |Y \ X|
                        //         add X ∩ Y to W
                        //     else
                        //         add Y \ X to W
                        if equivalence_classes_aux.contains(&equivalence_class) {
                            equivalence_classes_aux.remove(&equivalence_class);
                            equivalence_classes_aux.insert(intersection.clone());
                            equivalence_classes_aux.insert(difference.clone());
                        } else {
                            if intersection.len() <= difference.len() {
                                equivalence_classes_aux.insert(intersection.clone());
                            } else {
                                equivalence_classes_aux.insert(difference.clone());
                            }
                        }
                    }

                    equivalence_classes = equivalence_classes_vector.iter().cloned().collect();
                    i += 1;
                    // end;
                }
                // end;
            }
            // end;
        }
        //

        for equivalence_class in &equivalence_classes {
            for accept_state in &self.accept_states {
                if equivalence_class.contains(accept_state) {
                    accept_states.insert(set_to_state(equivalence_class));
                }
            }
        }

        for equivalence_class in &equivalence_classes {
            if equivalence_class.contains(&self.start_state) {
                start_state = set_to_state(equivalence_class);
            }
        }

        for equivalence_class in &equivalence_classes {
            if !equivalence_class.is_empty() {
                for letter in &alphabet {
                    let state_string = set_to_state(equivalence_class);
                    states.insert(state_string.clone());
                    let state = equivalence_class.iter().cloned().last().unwrap();

                    match self
                        .transition_function
                        .get(&(state.clone(), letter.clone()))
                    {
                        Some(out_state) => {
                            let mut out_state_equivalence_class = BTreeSet::new();

                            for equivalence_class in &equivalence_classes {
                                if equivalence_class.contains(out_state) {
                                    out_state_equivalence_class = equivalence_class.clone();
                                }
                            }

                            transition_function.insert(
                                (state_string.clone(), letter.clone()),
                                set_to_state(&out_state_equivalence_class),
                            );
                        }
                        None => (),
                    }
                }
            }
        }

        DeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: transition_function,
            start_state: start_state,
            accept_states: accept_states,
        }
    }

    pub fn minimize(&self) -> Self {
        let dfa = self.remove_unreachable_states();
        let dfa = dfa.remove_non_productive_states();
        dfa.remove_equivalent_states()
    }

    pub fn union(&self, other: &Self) -> Self {
        let self_as_nfa = NondeterministicFiniteAutomata::from(self);
        let other_as_nfa = NondeterministicFiniteAutomata::from(other);

        let union_as_nfa = self_as_nfa.union(&other_as_nfa);

        let union_as_dfa = DeterministicFiniteAutomata::from(&union_as_nfa);

        union_as_dfa.minimize()
    }

    pub fn complement(&self) -> Self {
        DeterministicFiniteAutomata {
            states: self.states.clone(),
            alphabet: self.alphabet.clone(),
            transition_function: self.transition_function.clone(),
            start_state: self.start_state.clone(),
            accept_states: self
                .states
                .difference(&self.accept_states)
                .cloned()
                .collect(),
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let self_complement = self.complement();
        let other_complement = other.complement();

        let self_as_nfa = NondeterministicFiniteAutomata::from(&self_complement);
        let other_as_nfa = NondeterministicFiniteAutomata::from(&other_complement);

        let union_as_nfa = self_as_nfa.union(&other_as_nfa);

        let union_as_dfa = DeterministicFiniteAutomata::from(&union_as_nfa);

        let union_complement = union_as_dfa.complement();

        union_complement.minimize()
    }
}

fn powerset<T: Clone>(slice: &[T]) -> Vec<Vec<T>> {
    let mut v: Vec<Vec<T>> = Vec::new();

    for mask in 0..(1 << slice.len()) {
        let mut ss: Vec<T> = vec![];
        let mut bitset = mask;
        while bitset > 0 {
            // isolate the rightmost bit to select one item
            let rightmost: u64 = bitset & !(bitset - 1);
            // turn the isolated bit into an array index
            let idx = rightmost.trailing_zeros();
            let item = (*slice.get(idx as usize).unwrap()).clone();
            ss.push(item);
            // zero the trailing bit
            bitset &= bitset - 1;
        }
        v.push(ss);
    }
    v
}

impl From<&NondeterministicFiniteAutomata> for DeterministicFiniteAutomata {
    fn from(automata: &NondeterministicFiniteAutomata) -> Self {
        let mut states = BTreeSet::new();
        let mut alphabet = BTreeSet::new();
        let mut transition_function = BTreeMap::new();
        let mut start_state;
        let mut accept_states = BTreeSet::new();

        for letter in &automata.alphabet {
            alphabet.insert(letter.clone());
        }

        {
            let mut state_set_vec = vec![];

            for state in &automata.states {
                state_set_vec.push(state.clone());
            }

            let power_state_set_vec = powerset(&state_set_vec);

            for vec in &power_state_set_vec {
                let set: BTreeSet<_> = vec.iter().cloned().collect();

                let state_name = set_to_state(&set);

                states.insert(state_name);
            }
        }

        let mut epsilon_closure = BTreeMap::new();

        {
            for state in &automata.states {
                let mut before = BTreeSet::new();

                before.insert(state.clone());

                let mut after = BTreeSet::new();

                let mut stop = false;
                while !stop {
                    after = BTreeSet::new();

                    for state in &before {
                        after.insert(state.clone());

                        match automata
                            .transition_function
                            .get(&(state.clone(), "&".to_string()))
                        {
                            Some(set) => {
                                for state in set {
                                    after.insert(state.clone());
                                }
                            }
                            None => (),
                        }
                    }

                    if after.is_subset(&before) && after.is_superset(&before) {
                        stop = true;
                    } else {
                        before = after.clone();
                    }
                }

                epsilon_closure.insert(state.clone(), after);
            }
        }

        for state in &states {
            for letter in &alphabet {
                let mut output_state_set = BTreeSet::new();

                let split_state_names = state_to_set(state);

                for state_name in split_state_names {
                    if automata.accept_states.contains(&state_name) {
                        accept_states.insert(state.clone());
                    }
                    match automata
                        .transition_function
                        .get(&(state_name.to_owned(), letter.clone()))
                    {
                        Some(set) => {
                            for state in set {
                                let epsilon_closure_for_state = epsilon_closure.get(state).unwrap();

                                for state in epsilon_closure_for_state {
                                    output_state_set.insert(state.clone());
                                }
                            }
                        }
                        None => (),
                    }
                }

                let output_state_name = set_to_state(&output_state_set);

                transition_function.insert((state.clone(), letter.clone()), output_state_name);
            }
        }

        {
            let start_state_closure = epsilon_closure.get(&automata.start_state).unwrap();

            start_state = set_to_state(start_state_closure);
        }

        DeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: transition_function,
            start_state: start_state,
            accept_states: accept_states,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_to_state() {
        let states = [
            "q0".to_string(),
            "q1".to_string(),
            "q2".to_string(),
            "q3".to_string(),
        ]
        .iter()
        .cloned()
        .collect();

        let state = super::set_to_state(&states);

        assert_eq!(state, String::from("(q0, q1, q2, q3)"));

        let states = [
            "".to_string(),
            "1".to_string(),
            "(2, 3)".to_string(),
            "(2, 3, (2))".to_string(),
        ]
        .iter()
        .cloned()
        .collect();

        let state = super::set_to_state(&states);

        assert_eq!(state, String::from("(, (2, 3), (2, 3, (2)), 1)"));
    }

    #[test]
    fn state_to_set() {
        let state = String::from("");

        let set = super::state_to_set(&state);

        let correct_set = [state].iter().cloned().collect();

        assert_eq!(set, correct_set);

        let state = String::from("q0");

        let set = super::state_to_set(&state);

        let correct_set = [state].iter().cloned().collect();

        assert_eq!(set, correct_set);

        let state = String::from("()");

        let set = super::state_to_set(&state);

        let correct_set = [String::from("")].iter().cloned().collect();

        assert_eq!(set, correct_set);

        let state = String::from("(q0)");

        let set = super::state_to_set(&state);

        let correct_set = [String::from("q0")].iter().cloned().collect();

        assert_eq!(set, correct_set);

        let state = String::from("(q0, q1)");

        let set = super::state_to_set(&state);

        let correct_set = [String::from("q0"), String::from("q1")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(set, correct_set);

        let state = String::from("(())");

        let set = super::state_to_set(&state);

        let correct_set = [String::from("()")].iter().cloned().collect();

        assert_eq!(set, correct_set);

        let state = String::from("((q0))");

        let set = super::state_to_set(&state);

        let correct_set = [String::from("(q0)")].iter().cloned().collect();

        assert_eq!(set, correct_set);

        let state = String::from("((q0), (q1))");

        let set = super::state_to_set(&state);

        let correct_set = [String::from("(q0)"), String::from("(q1)")]
            .iter()
            .cloned()
            .collect();

        assert_eq!(set, correct_set);

        let state = String::from("((q0), (q1), q2)");

        let set = super::state_to_set(&state);

        let correct_set = [
            String::from("(q0)"),
            String::from("(q1)"),
            String::from("q2"),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(set, correct_set);

        let state = String::from("((q0, q1, (q2)), (q1, (q2, q3, (q3))), q2)");

        let set = super::state_to_set(&state);

        let correct_set = [
            String::from("(q0, q1, (q2))"),
            String::from("(q1, (q2, q3, (q3)))"),
            String::from("q2"),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(set, correct_set);
    }

    #[test]
    fn remove_unreachable_states() {
        let mut hash = BTreeMap::new();

        hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
        hash.insert((String::from("q1"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q1"), String::from("b")), String::from("q1"));
        hash.insert((String::from("q2"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q2"), String::from("b")), String::from("q1"));
        hash.insert((String::from("q3"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q3"), String::from("b")), String::from("q1"));

        let states = [
            "q0".to_string(),
            "q1".to_string(),
            "q2".to_string(),
            "q3".to_string(),
        ]
        .iter()
        .cloned()
        .collect();

        let alphabet = ["a".to_string(), "b".to_string()].iter().cloned().collect();

        let accept_states = [String::from("q0"), String::from("q3")]
            .iter()
            .cloned()
            .collect();

        let automata = DeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: hash,
            start_state: String::from("q0"),
            accept_states: accept_states,
        };

        let automata = automata.remove_unreachable_states();

        let result_states: BTreeSet<_> = ["q0".to_string(), "q1".to_string()]
            .iter()
            .cloned()
            .collect();

        assert_eq!(automata.states, result_states);

        let result_alphabet = ["a".to_string(), "b".to_string()].iter().cloned().collect();

        assert_eq!(automata.alphabet, result_alphabet);

        let mut result_hash = BTreeMap::new();

        result_hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
        result_hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
        result_hash.insert((String::from("q1"), String::from("a")), String::from("q0"));
        result_hash.insert((String::from("q1"), String::from("b")), String::from("q1"));

        assert_eq!(automata.transition_function, result_hash);

        let result_start_state = String::from("q0");

        assert_eq!(automata.start_state, result_start_state);

        let accept_states_result = [String::from("q0")].iter().cloned().collect();

        assert_eq!(automata.accept_states, accept_states_result);
    }

    #[test]
    fn remove_non_productive_states() {
        let mut hash = BTreeMap::new();

        hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
        hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
        hash.insert((String::from("q1"), String::from("a")), String::from("q2"));
        hash.insert((String::from("q1"), String::from("b")), String::from("q0"));
        hash.insert((String::from("q2"), String::from("a")), String::from("q3"));
        hash.insert((String::from("q2"), String::from("b")), String::from("q4"));
        hash.insert((String::from("q3"), String::from("a")), String::from("q4"));
        hash.insert((String::from("q3"), String::from("b")), String::from("q3"));
        hash.insert((String::from("q4"), String::from("a")), String::from("q3"));
        hash.insert((String::from("q4"), String::from("b")), String::from("q4"));

        let states = [
            "q0".to_string(),
            "q1".to_string(),
            "q2".to_string(),
            "q3".to_string(),
            "q4".to_string(),
        ]
        .iter()
        .cloned()
        .collect();

        let alphabet = ["a".to_string(), "b".to_string()].iter().cloned().collect();

        let accept_states = [String::from("q0")].iter().cloned().collect();

        let automata = DeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: hash,
            start_state: String::from("q0"),
            accept_states: accept_states,
        };

        let automata = automata.remove_non_productive_states();

        let result_states: BTreeSet<_> = ["q0".to_string(), "q1".to_string()]
            .iter()
            .cloned()
            .collect();

        assert_eq!(automata.states, result_states);

        let result_alphabet = ["a".to_string(), "b".to_string()].iter().cloned().collect();

        assert_eq!(automata.alphabet, result_alphabet);

        let mut result_hash = BTreeMap::new();

        result_hash.insert((String::from("q0"), String::from("a")), String::from("q0"));
        result_hash.insert((String::from("q0"), String::from("b")), String::from("q1"));
        result_hash.insert((String::from("q1"), String::from("b")), String::from("q0"));

        assert_eq!(automata.transition_function, result_hash);

        let result_start_state = String::from("q0");

        assert_eq!(automata.start_state, result_start_state);

        let accept_states_result = [String::from("q0")].iter().cloned().collect();

        assert_eq!(automata.accept_states, accept_states_result);
    }

    #[test]
    fn remove_equivalent_states() {
        let mut hash = BTreeMap::new();

        hash.insert((String::from("A"), String::from("0")), String::from("B"));
        hash.insert((String::from("A"), String::from("1")), String::from("C"));
        hash.insert((String::from("B"), String::from("0")), String::from("A"));
        hash.insert((String::from("B"), String::from("1")), String::from("D"));
        hash.insert((String::from("C"), String::from("0")), String::from("E"));
        hash.insert((String::from("C"), String::from("1")), String::from("F"));
        hash.insert((String::from("D"), String::from("0")), String::from("E"));
        hash.insert((String::from("D"), String::from("1")), String::from("F"));
        hash.insert((String::from("E"), String::from("0")), String::from("E"));
        hash.insert((String::from("E"), String::from("1")), String::from("F"));
        hash.insert((String::from("F"), String::from("0")), String::from("F"));
        hash.insert((String::from("F"), String::from("1")), String::from("F"));

        let states = [
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
        ]
        .iter()
        .cloned()
        .collect();

        let alphabet = ["0".to_string(), "1".to_string()].iter().cloned().collect();

        let accept_states = [String::from("C"), String::from("D"), String::from("E")]
            .iter()
            .cloned()
            .collect();

        let automata = DeterministicFiniteAutomata {
            states: states,
            alphabet: alphabet,
            transition_function: hash,
            start_state: String::from("A"),
            accept_states: accept_states,
        };

        let automata = automata.remove_equivalent_states();

        let states_result: BTreeSet<_> = [
            "(A, B)".to_string(),
            "(C, D, E)".to_string(),
            "(F)".to_string(),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(automata.states, states_result);

        let result_alphabet = ["0".to_string(), "1".to_string()].iter().cloned().collect();

        assert_eq!(automata.alphabet, result_alphabet);

        let mut result_hash = BTreeMap::new();

        result_hash.insert(
            (String::from("(A, B)"), String::from("0")),
            String::from("(A, B)"),
        );
        result_hash.insert(
            (String::from("(A, B)"), String::from("1")),
            String::from("(C, D, E)"),
        );
        result_hash.insert(
            (String::from("(C, D, E)"), String::from("0")),
            String::from("(C, D, E)"),
        );
        result_hash.insert(
            (String::from("(C, D, E)"), String::from("1")),
            String::from("(F)"),
        );
        result_hash.insert(
            (String::from("(F)"), String::from("0")),
            String::from("(F)"),
        );
        result_hash.insert(
            (String::from("(F)"), String::from("1")),
            String::from("(F)"),
        );

        assert_eq!(automata.transition_function, result_hash);

        let result_start_state = String::from("(A, B)");

        assert_eq!(automata.start_state, result_start_state);

        let accept_states_result = [String::from("(C, D, E)")].iter().cloned().collect();

        assert_eq!(automata.accept_states, accept_states_result);
    }
}
