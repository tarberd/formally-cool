use super::cli_util::*;
use super::cli_dfa::dfa_menu;
use formally_cool::regular_languages::NondeterministicFiniteAutomata;
use formally_cool::regular_languages::DeterministicFiniteAutomata;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;

pub fn create_nfa() {
    let name = ask("name?".to_string(), false);
    let alpha = ask("alphabet? (format: 'symbol' 'symbol'*)".to_string(), false);
    let symbols:HashSet<String> = alpha.split_whitespace().map(|s| s.to_string()).collect();

    let mut states = ask("states? (format: 'initial state' 'state'* || 'number of states')".to_string(), false);
    let mut statesvec:Vec<String> = states.split_whitespace().map(|s| s.to_string()).collect();
    let mut initstate = String::new();
    if statesvec.len() == 1 && statesvec[0].parse::<i32>().is_ok() {
        let num_states:i32 = statesvec[0].parse().unwrap();
        statesvec[0] = "q0".to_string();
        initstate = "q0".to_string();
        for i in 1 .. (num_states) {
            let ststr = "q".to_string() + &i.to_string();
            statesvec.push(ststr);
        }
    } else {
        initstate = statesvec[0].clone();
    }
    let mut stateshash:HashSet<String> = statesvec.iter().cloned().collect();

    let mut accstates = ask("accept states? (format: 'state'*), a state will be created if it does not exist".to_string(), true);
    let mut accstatesvec:Vec<String> = accstates.split_whitespace().map(|s| s.to_string()).collect();

    let accstateshash:HashSet<String> = accstatesvec.iter().cloned().collect();
    for str in accstateshash.iter() {
        stateshash.insert(str.clone());
    }

    let mut transitionsmap:BTreeMap<(String, String), BTreeSet<String>> = BTreeMap::new();
    let mut input = ask("transition? (format: 'state' 'symbol' 'next_state' || exit)".to_string(), true);

    while input.trim() != "exit" && input.len() != 0 {
        let t:Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        if t.len() > 2{
            let mut t0 = t[0].to_string().clone();
            let mut t1 = t[1].to_string().clone();
            let mut t2 = t[2].to_string().clone();
            if transitionsmap.contains_key(&(t0.clone(), t1.clone())) {
                transitionsmap.get_mut(&(t0, t1)).unwrap().insert(t2);
            } else {
                let mut obj = BTreeSet::new();
                obj.insert(t[2].to_string().clone());
                transitionsmap.insert((t0, t1), obj);
            }
        }
        input = ask("transition? (format: 'state' 'symbol' 'next_state' || exit)".to_string(), true);
    }



    let mut automata = NondeterministicFiniteAutomata {
        states: stateshash
            .iter()
            .cloned()
            .collect(),
        alphabet: symbols
            .iter()
            .cloned()
            .collect(),
        transition_function: transitionsmap,
        start_state: initstate,
        accept_states: accstateshash
            .iter()
            .cloned()
            .collect(),
    };
    nfa_menu(&mut automata, name);
}
fn edit_alphabet(automata: &mut NondeterministicFiniteAutomata) {
    let mut running = true;
    while(running) {
        automata.printTable();
        let mut option = ask("back | add | remove (won't remove if alphabet would become empty)".to_string(), false);
        if option.trim() == "add" {
            let letter = ask("symbol?".to_string(), false);
            automata.alphabet.insert(letter.clone().to_string());
        } else if option.trim() == "remove" {
            let letter = ask("symbol?".to_string(), false);
            if automata.alphabet.len() > 1 {
                automata.removeSymbol(&letter.clone().to_string());
            }
        }
    }
}
fn edit_transition(automata: &mut NondeterministicFiniteAutomata) {
    let mut running = true;
    while(running) {
        automata.printTable();
        let mut option = ask("add | remove | back".to_string(), false);
        if option.trim() == "add" {
            let mut input = ask("transition? (format: 'state' 'symbol' 'next_state')".to_string(), false);
            let t:Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
            if t.len() > 2{
                let mut t0 = t[0].to_string().clone();
                let mut t1 = t[1].to_string().clone();
                let mut t2 = t[2].to_string().clone();
                if automata.transition_function.contains_key(&(t0.clone(), t1.clone())) {
                    automata.transition_function.get_mut(&(t0, t1)).unwrap().insert(t2);
                } else {
                    let mut obj = BTreeSet::new();
                    obj.insert(t[2].to_string().clone());
                    automata.transition_function.insert((t0, t1), obj);
                }
            }
        } else if option.trim() == "remove" {
            let mut input = ask("transition? (format: 'state' 'symbol' 'next_state')".to_string(), false);
            let t:Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
            if t.len() > 2{
                let mut t0 = t[0].to_string().clone();
                let mut t1 = t[1].to_string().clone();
                let mut t2 = t[2].to_string().clone();
                if automata.transition_function.contains_key(&(t0.clone(), t1.clone())) {
                    automata.transition_function.get_mut(&(t0.clone(), t1.clone())).unwrap().remove(&t2);
                    if automata.transition_function[&(t0.clone(), t1.clone())].len() == 0 {
                        automata.transition_function.remove(&(t0.clone(), t1.clone()));
                    }
                }
            }
        } else if option.trim() == "back" {
            running = false;
        }
    }
}
fn edit_state(automata: &mut NondeterministicFiniteAutomata) {
    let mut running = true;
    while(running) {
        automata.printTable();
        let mut option = ask("back | add | remove | accept | disaccept | initial".to_string(), false);
        if option.trim() == "add" {
            let state = ask("state?".to_string(), false);
            automata.states.insert(state.clone().to_string());
        } else if option.trim() == "remove" {
            let state = ask("state?".to_string(), false);
            automata.removeState(&state.clone().to_string());
        } else if option.trim() == "accept" {
            let state = ask("state?".to_string(), false);
            automata.states.insert(state.clone().to_string());
            automata.accept_states.insert(state.clone().to_string());
        } else if option.trim() == "disaccept" || option.trim().len() == 0 {
            let state = ask("state?".to_string(), false);
            automata.accept_states.remove(&state.clone().to_string());
        } else if option.trim() == "initial (set new initial state)?" || option.trim().len() == 0 {
            let state = ask("state?".to_string(), false);
            automata.start_state = state.clone().to_string();
            automata.states.insert(state.clone().to_string());
        } else if option.trim() == "back" {
            running = false;
        }
    }
}
fn nfa_edit(mut automata: &mut NondeterministicFiniteAutomata) {
    let mut running = true;
    while(running) {
        automata.printTable();
        let mut option = ask("back | state | transition | alphabet | convert (to dfa)".to_string(), false);
        if option.trim() == "state" {
            edit_state(&mut automata);
        } else if option.trim() == "transition" {
            edit_transition(&mut automata);
        } else if option.trim() == "alphabet" {
            edit_alphabet(&mut automata);
        } else if option.trim() == "back" || option.trim().len() == 0 {
            running = false;
        } else if option.trim() == "convert" {
            let name = ask("name?".to_string(), false);
            let auto = automata.clone();
            let mut dfa = DeterministicFiniteAutomata::from(&auto);
            dfa_menu(&mut dfa, name.clone());
            running = false;
        }
    }
}
pub fn nfa_menu (automata: &mut NondeterministicFiniteAutomata, name:String){
    let mut running = true;
    while (running) {
        automata.printTable();
        let mut option = ask("back | save | edit | compute".to_string(), false);
        if option.trim() == "save" {
            save(serde_yaml::to_string(&automata).unwrap(), name.clone() + ".nfa");
        } else if option.trim() == "edit" {
            nfa_edit(automata);
        } else if option.trim() == "back" {
            running = false;
        } else if option.trim() == "compute" {

        }
    }
}
pub fn load_nfa () {
    let name = ask("name?".to_string(), false);
    let serialized = open(name.clone() + ".nfa");
    let mut automata: NondeterministicFiniteAutomata = serde_yaml::from_str(&serialized).unwrap();
    nfa_menu(&mut automata, name);
}
