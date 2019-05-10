mod cli_util;
use cli_util::*;
mod cli_dfa;
use cli_dfa::*;
mod cli_nfa;
use cli_nfa::*;
mod cli_rg;
use cli_rg::*;
mod cli_regex;
use cli_regex::*;

use formally_cool::regular_languages::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;


fn create (){
    let mut running = true;
    while running {
        let mut input = ask("dfa | nfa | rg | regex | back".to_string(), false);
        running = false;
        if input.trim() == "dfa" {
            create_dfa();
        } else if input.trim() == "nfa" {
            create_nfa();
        } else if input.trim() == "rg" {
            create_rg();
        } else if input.trim() == "regex" {
            create_regex();
        } else if input.trim() == "back" {
        } else {
            running = true;
        }
    }
}

fn load() {
    let mut running = true;
    while running {
        let mut input = ask("dfa | nfa | rg | regex | back".to_string(), false);
        running = false;
        if input.trim() == "dfa" {
            load_dfa();
        } else if input.trim() == "nfa" {
            load_nfa();
        } else if input.trim() == "rg" {
        } else if input.trim() == "regex" {
            load_regex();
        } else if input.trim() == "back" {
        } else {
            running = true;
        }
    }
}

fn main() {
    println!("formally-cli interface:");
    let mut running = true;
    while running {
        let input = ask("exit | create | load".to_string(), false);
        if input.trim() == "create" {
            create();
        } else if input.trim() == "load" {
            load();
        } else if input.trim() == "exit" {
            running = false;
        }
    }
}
