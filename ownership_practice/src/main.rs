fn main() {
    let sushi = "salmon";
    let dinner = sushi;
    println!("My dinner is {dinner} and it is {sushi}");

    let prawn = String::from("prawn");
    eat_meal(prawn);
}
fn eat_meal(mut meal: String) {
    meal.clear();
}