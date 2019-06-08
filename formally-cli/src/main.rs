mod cli_util;
use cli_util::*;
mod cli_dfa;
use cli_dfa::*;
mod cli_nfa;
use cli_nfa::*;
mod cli_rg;
use cli_rg::*;
mod cli_cfg;
use cli_cfg::*;


fn create (){
    let mut running = true;
    while running {
        let input = ask("dfa | nfa | rg | cfg | back".to_string(), false);
        running = false;
        if input.trim() == "dfa" {
            create_dfa();
        } else if input.trim() == "nfa" {
            create_nfa();
        } else if input.trim() == "rg" {
            create_rg();
        } else if input.trim() == "cfg" {
            create_cfg();
        } else if input.trim() == "back" {
        } else {
            running = true;
        }
    }
}

fn load() {
    let mut running = true;
    while running {
        let input = ask("dfa | nfa | rg | cfg | back".to_string(), false);
        running = false;
        if input.trim() == "dfa" {
            load_dfa();
        } else if input.trim() == "nfa" {
            load_nfa();
        } else if input.trim() == "rg" {
            load_rg();
        } else if input.trim() == "cfg" {
            load_cfg();
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
