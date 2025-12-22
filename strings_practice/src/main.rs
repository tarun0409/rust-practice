use std::io::{self, Read};
fn make_money(txt: &mut String) {
    txt.push_str("$$$$");
}
fn trim_and_capitalize(txt: String) -> String {
    txt.trim().to_uppercase().to_string()
}
fn elements<'a>(txt: &'a str) -> Vec<&'a str> {
    txt.split('!').collect()
}
fn get_identity() -> String {
    let mut first_name =  String::new();
    let mut last_name = String:: new();
    println!("Enter first name: ");
    io::stdin().read_line(&mut first_name).expect("Some error occurred while getting first name");
    println!("Enter last name: ");
    io::stdin().read_line(&mut last_name).expect("Some error occurred while getting last name");
    return format!("{} {}", first_name.trim(), last_name.trim());
}
fn main() {
    let mut txt = String::from("Lets roll");
    make_money(&mut txt);
    println!("{txt}");
    println!("{}", trim_and_capitalize(txt));
    println!("{:?}", elements("India!Pakistan!Bangladesh"));
    println!("Your name is {}", get_identity().trim());
}
