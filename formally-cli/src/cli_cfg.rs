use super::cli_util::*;
use super::cli_nfa::nfa_menu;
use formally_cool::regular_languages::Grammar;
use formally_cool::regular_languages::NondeterministicFiniteAutomata;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn create_cfg() {
    let name = ask("name?".to_string(), false);
    let mut startvariable = String::new();
    let mut variableset = BTreeSet::new();
    let mut terminalset = BTreeSet::new();
    let mut prodrules:BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let initprod = ask("initial production?
    \n(format: 'initial symbol' 'production' 'production'*)".to_string(), false);
    let prodvec:Vec<String> = initprod.split_whitespace().map(|s| s.to_string()).collect();
    if prodvec.len() > 1 {
        startvariable = prodvec[0].clone();
        variableset.insert(startvariable.clone().trim().to_string());
        let mut obj = BTreeSet::new();
        for i in 1..prodvec.len(){
            obj.insert(prodvec[i].clone().trim().to_string());
            let l = prodvec[i].clone().to_string().chars().next().unwrap().to_string();
            if !variableset.contains(&l.clone()) {
                terminalset.insert(l.clone());
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
                    prodrules.get_mut(&t[0].clone()).unwrap().insert(t[i].clone());
                }
            } else {
                let mut obj = BTreeSet::new();
                for i in 1..t.len() {
                    obj.insert(t[i].clone());
                }
                prodrules.insert(t[0].clone(), obj);
            }
        }
        prod = ask("production? (format: 'symbol' 'production' 'production'*) || exit".to_string(), true);
    }
    for (_, p) in prodrules.clone() {
        for a in p {
            let l = a.clone().to_string().chars().next().unwrap().to_string();
            if !variableset.contains(&l.clone()) {
                terminalset.insert(l.clone());
            }
        }
    }


    let mut contextfree_grammar = Grammar{
        variables: variableset,
        terminals: terminalset,
        rules: prodrules,
        start_variable: startvariable,
    };
    cfg_menu(&mut contextfree_grammar, name);
}
fn cfg_edit(contextfree_grammar: &mut Grammar) {
    let mut running = true;
    while running {
        contextfree_grammar.printTable();
        let option = ask("back | add (rules) | remove (rules) | convert (to nfa)".to_string(), false);
        if option.trim() == "add" {
            let prod = ask("production? (format: 'symbol' 'production' 'production'*)".to_string(), true);
            let t:Vec<String> = prod.split_whitespace().map(|s| s.to_string()).collect();
            if t.len() > 1{
                contextfree_grammar.variables.insert(t[0].clone());
                if contextfree_grammar.rules.contains_key(&t[0].clone()){
                    for i in 1..t.len() {
                        contextfree_grammar.rules.get_mut(&t[0].clone()).unwrap().insert(t[i].clone());
                        contextfree_grammar.variables.insert(t[i].clone());
                    }
                } else {
                    let mut obj = BTreeSet::new();
                    for i in 0..t.len() {
                        obj.insert(t[i].clone());
                        contextfree_grammar.variables.insert(t[i].clone());
                    }
                    contextfree_grammar.rules.insert(t[0].clone(), obj);
                }
            }
        }else if option.trim() == "remove" {
            let prod = ask("production? (format: 'symbol' 'production' 'production'*)".to_string(), true);
            let t:Vec<String> = prod.split_whitespace().map(|s| s.to_string()).collect();
            if t.len() > 1{
                contextfree_grammar.variables.insert(t[0].clone());
                if contextfree_grammar.rules.contains_key(&t[0].clone()){
                    for i in 1..t.len() {
                        contextfree_grammar.rules.get_mut(&t[0].clone()).unwrap().insert(t[i].clone());
                    }
                } else {
                    let mut obj = BTreeSet::new();
                    for i in 1..t.len() {
                        obj.insert(t[i].clone());
                    }
                    contextfree_grammar.rules.insert(t[0].clone(), obj);
                }
            }
        } else if option.trim() == "back" || option.trim().len() == 0 {
            running = false;
        } else if option.trim() == "convert" {
            let name = ask("name?".to_string(), false);
            let cfg = contextfree_grammar.clone();
            let mut nfa = NondeterministicFiniteAutomata::from(&cfg);
            nfa_menu(&mut nfa, name.clone());
            running = false;
        }
    }
}
pub fn cfg_menu(contextfree_grammar: &mut Grammar, name : String) {
    let mut running = true;
    while  running {
        contextfree_grammar.printTable();
        let option = ask("back | save | edit".to_string(), false);
        if option.trim() == "save" {
            save(serde_yaml::to_string(&contextfree_grammar).unwrap(), name.clone() + ".cfg");
        } else if option.trim() == "edit" {
            cfg_edit(contextfree_grammar);
        } else if option.trim() == "back" {
            running = false;
        }
    }
}
pub fn load_cfg () {
    let name = ask("name?".to_string(), false);
    let serialized = open(name.clone() + ".cfg");
    let mut cfg: Grammar = serde_yaml::from_str(&serialized).unwrap();
    cfg_menu(&mut cfg, name);
}
