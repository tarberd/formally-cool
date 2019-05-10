use super::cli_util::*;
use formally_cool::regular_languages::RegularGrammar;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn create_rg() {
    let name = ask("name?".to_string(), false);
    let mut startvariable = String::new();
    let mut variableset = BTreeSet::new();
    let mut prodrules:BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let initprod = ask("initial production?
    \n(format: 'initial symbol' 'production' 'production'*
    \n'production' = 'terminal' || 'terminal''nonterminal'".to_string(), false);
    let mut prodvec:Vec<String> = initprod.split_whitespace().map(|s| s.to_string()).collect();
    if prodvec.len() > 1 {
        startvariable = prodvec[0].clone();
        variableset.insert(startvariable.clone());
        let mut obj = BTreeSet::new();
        for i in 1..prodvec.len(){
            obj.insert(prodvec[i].clone());
        }
        prodrules.insert(startvariable.clone(), obj);
    }
    let mut prod = ask("production? (format: 'symbol' 'production' 'production'*) || exit".to_string(), true);

    while prod.trim() != "exit" && prod.trim().len() > 1{
        let t:Vec<String> = prod.split_whitespace().map(|s| s.to_string()).collect();
        if t.len() > 1{
            variableset.insert(t[0].clone());
            if prodrules.contains_key(&t[0].clone()){
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
    let mut regular_grammar = RegularGrammar{
        variables: variableset,
        terminals: BTreeSet::new(),
        rules: prodrules,
        start_variable: startvariable,
    };
    rg_menu(&mut regular_grammar, name);
}
pub fn rg_menu(regular_grammar: &mut RegularGrammar, name : String) {
    
}
