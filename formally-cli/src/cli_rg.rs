use super::cli_util::*;
use super::cli_nfa::nfa_menu;
use formally_cool::regular_languages::RegularGrammar;
use formally_cool::regular_languages::NondeterministicFiniteAutomata;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::str::from_utf8;

pub fn create_rg() {
    let name = ask("name?".to_string(), false);
    let mut startvariable = String::new();
    let mut variableset = BTreeSet::new();
    let mut terminalset = BTreeSet::new();
    let mut prodrules:BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let initprod = ask("initial production?
    \n(format: 'initial symbol' 'production' 'production'*
    \n'production' = 'terminal' || 'terminal''nonterminal'".to_string(), false);
    let mut prodvec:Vec<String> = initprod.split_whitespace().map(|s| s.to_string()).collect();
    if prodvec.len() > 1 {
        startvariable = prodvec[0].clone();
        variableset.insert(startvariable.clone().trim().to_string());
        let mut obj = BTreeSet::new();
        for i in 1..prodvec.len(){
            obj.insert(prodvec[i].clone().trim().to_string());
            if prodvec[i].len() == 2 {
                terminalset.insert(prodvec[i].clone().to_string().chars().next().unwrap().to_string());
                variableset.insert(from_utf8(&[prodvec[i].clone().to_string().as_bytes()[1]]).unwrap().to_string());
            } else {
                terminalset.insert(prodvec[i].clone().to_string().chars().next().unwrap().to_string());
            }
        }
        prodrules.insert(startvariable.clone().trim().to_string(), obj);
    }
    let mut prod = ask("production? (format: 'symbol' 'production' 'production'*) || exit".to_string(), true);

    while prod.trim() != "exit" && prod.trim().len() > 1{
        let t:Vec<String> = prod.split_whitespace().map(|s| s.to_string()).collect();
        if t.len() > 1{
            variableset.insert(t[0].clone().trim().to_string());
            if prodrules.contains_key(&t[0].clone().trim().to_string()){
                for i in 1..t.len() {
                    if t[i].len() == 2 {
                        terminalset.insert(t[i].clone().to_string().chars().next().unwrap().to_string());
                        variableset.insert(from_utf8(&[t[i].clone().to_string().as_bytes()[1]]).unwrap().to_string());
                    } else {
                        terminalset.insert(t[i].clone().to_string().chars().next().unwrap().to_string());
                    }
                    prodrules.get_mut(&t[0].clone()).unwrap().insert(t[i].clone());
                }
            } else {
                let mut obj = BTreeSet::new();
                for i in 1..t.len() {
                    obj.insert(t[i].clone());
                    if t[i].len() == 2 {
                        terminalset.insert(t[i].clone().to_string().chars().next().unwrap().to_string());
                        variableset.insert(from_utf8(&[t[i].clone().to_string().as_bytes()[1]]).unwrap().to_string());
                    } else {
                        terminalset.insert(t[i].clone().to_string().chars().next().unwrap().to_string());
                    }
                }
                prodrules.insert(t[0].clone(), obj);
            }
        }
        prod = ask("production? (format: 'symbol' 'production' 'production'*) || exit".to_string(), true);
    }
    let mut regular_grammar = RegularGrammar{
        variables: variableset,
        terminals: terminalset,
        rules: prodrules,
        start_variable: startvariable,
    };
    rg_menu(&mut regular_grammar, name);
}
fn rg_edit(mut regular_grammar: &mut RegularGrammar) {
    let mut running = true;
    while(running) {
        regular_grammar.printTable();
        let mut option = ask("back | add (rules) | remove (rules) | convert (to nfa)".to_string(), false);
        if option.trim() == "add" {
            let mut prod = ask("production? (format: 'symbol' 'production' 'production'*)".to_string(), true);
            let t:Vec<String> = prod.split_whitespace().map(|s| s.to_string()).collect();
            if t.len() > 1{
                regular_grammar.variables.insert(t[0].clone());
                if regular_grammar.rules.contains_key(&t[0].clone()){
                    for i in 1..t.len() {
                        regular_grammar.rules.get_mut(&t[0].clone()).unwrap().insert(t[i].clone());
                        regular_grammar.variables.insert(t[i].clone());
                    }
                } else {
                    let mut obj = BTreeSet::new();
                    for i in 0..t.len() {
                        obj.insert(t[i].clone());
                        regular_grammar.variables.insert(t[i].clone());
                    }
                    regular_grammar.rules.insert(t[0].clone(), obj);
                }
            }
        }else if option.trim() == "remove" {
            let mut prod = ask("production? (format: 'symbol' 'production' 'production'*)".to_string(), true);
            let t:Vec<String> = prod.split_whitespace().map(|s| s.to_string()).collect();
            if t.len() > 1{
                regular_grammar.variables.insert(t[0].clone());
                if regular_grammar.rules.contains_key(&t[0].clone()){
                    for i in 1..t.len() {
                        regular_grammar.rules.get_mut(&t[0].clone()).unwrap().insert(t[i].clone());
                    }
                } else {
                    let mut obj = BTreeSet::new();
                    for i in 1..t.len() {
                        obj.insert(t[i].clone());
                    }
                    regular_grammar.rules.insert(t[0].clone(), obj);
                }
            }
        } else if option.trim() == "back" || option.trim().len() == 0 {
            running = false;
        } else if option.trim() == "convert" {
            let name = ask("name?".to_string(), false);
            let rg = regular_grammar.clone();
            let mut nfa = NondeterministicFiniteAutomata::from(&rg);
            nfa_menu(&mut nfa, name.clone());
            running = false;
        }
    }
}
pub fn rg_menu(regular_grammar: &mut RegularGrammar, name : String) {
    let mut running = true;
    while (running) {
        regular_grammar.printTable();
        let mut option = ask("back | save | edit".to_string(), false);
        if option.trim() == "save" {
            save(serde_yaml::to_string(&regular_grammar).unwrap(), name.clone() + ".rg");
        } else if option.trim() == "edit" {
            rg_edit(regular_grammar);
        } else if option.trim() == "back" {
            running = false;
        }
    }
}
pub fn load_rg () {
    let name = ask("name?".to_string(), false);
    let serialized = open(name.clone() + ".rg");
    let mut rg: RegularGrammar = serde_yaml::from_str(&serialized).unwrap();
    rg_menu(&mut rg, name);
}
