fn double_the_length<T>(vec: &Vec<T>) -> usize {
    vec.len()*2
}
fn last_two<T>(sl: &[T]) -> &[T] {
    let l = sl.len();
    &sl[l-2..l]
}
fn first_five<'a>(text: &'a str, announcement: &'a str) -> &'a str {
    println!("{announcement}");
    &text[..5]
}
fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &'a str) -> &'a str {
    if first.contains(target) {
        return first;
    }
    second
}
fn main() {
    let v = vec![String::from("Hello"), String::from("World"), String::from("Bonjour"), String::from("Paris")];
    println!("{}", double_the_length(&v));
    println!("{:?}", last_two(&v));
    println!("{}", first_five("Vanakkam Tamizha", "Naalai nam naadu"));
    println!("{}", find_string_that_has_content("Je m'appelle Tarun", "Ich bin Marc", "Tarun"));
}
