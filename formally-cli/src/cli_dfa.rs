use super::cli_util::*;
use formally_cool::regular_languages::DeterministicFiniteAutomata;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;

pub fn create_dfa() {
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

    let mut transitionsmap = BTreeMap::new();
    let mut input = ask("transition? (format: 'state' 'symbol' 'next_state' || exit)".to_string(), true);

    while input.trim() != "exit" && input.len() != 0 {
        let t:Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        if t.len() > 2{
            transitionsmap.insert((t[0].clone(), t[1].clone()), t[2].clone());
        }
        input = ask("transition? (format: 'state' 'symbol' 'next_state' || exit)".to_string(), true);
    }

    let mut automata = DeterministicFiniteAutomata {
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
    dfa_menu(&mut automata, name);
}

fn dfa_edit(automata: &mut DeterministicFiniteAutomata) {

}
fn dfa_menu (automata: &mut DeterministicFiniteAutomata, name: String){
    let mut running = true;
    while (running) {
        automata.printTable();
        let mut option = ask("back | save | edit".to_string(), false);
        if option.trim() == "save" {
            save(serde_yaml::to_string(&automata).unwrap(), name.clone() + ".dfa");
        } else if option.trim() == "edit" {
            dfa_edit(automata);
        } else if option.trim() == "back" {
            running = false;
        }
    }
}

pub fn load_dfa () {
    let name = ask("name?".to_string(), false);
    let serialized = open(name.clone() + ".dfa");
    let mut automata: DeterministicFiniteAutomata = serde_yaml::from_str(&serialized).unwrap();
    dfa_menu(&mut automata, name);
}
