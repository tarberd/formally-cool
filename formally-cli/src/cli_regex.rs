use super::cli_util::*;


pub fn create_regex() {
    let name = ask("name?".to_string(), false);
    let mut regex:Vec<char> = Vec::new();
    let str = ask("expression? (* : closure) (+ : union)".to_string(), false);
    for i in str.trim().chars() {
        if (i != ' '){
            regex.push(i.clone());
        }
    }

    regex_menu(&mut regex, name);
}
fn regex_edit(regex : &mut Vec<char>) {
    let str = ask("expression? (* : closure) (+ : union)".to_string(), false);
    regex.clear();
    for i in str.trim().chars() {
        if (i != ' '){
            regex.push(i.clone());
        }
    }
}
fn regex_menu(regex : &mut Vec<char>, name : String) {
    let mut running = true;
    while (running) {
        let mut option = ask("back | save | edit".to_string(), false);
        if option.trim() == "save" {
            save(serde_yaml::to_string(&regex).unwrap(), name.clone() + ".regex");
        } else if option.trim() == "edit" {
            regex_edit(regex);
        } else if option.trim() == "back" {
            running = false;
        }
    }
}
pub fn load_regex() {
    let name = ask("name?".to_string(), false);
    let serialized = open(name.clone() + ".regex");
    let mut regex: Vec<char> = serde_yaml::from_str(&serialized).unwrap();
    regex_menu(&mut regex, name);
}
