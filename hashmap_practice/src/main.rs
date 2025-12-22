use std::collections::HashMap;
fn main() {
    let data = [("Ketchup", vec!["French Fries", "Burgers", "Hot dog"]), ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"])];
    let mut sauces_to_meals = HashMap::from(data);
    sauces_to_meals.insert("Mustard", vec!["Hot dog","Burger", "Pretzel"]);

    match sauces_to_meals.remove("Mayonnaise"){
        Some(meals) => println!("{:?}", meals),
        None => println!("Invalid sauce"),
    }
    match sauces_to_meals.get("Mustard") {
        Some(meals) => println!("{:?}", meals),
        None => println!("Invalid sauce"),
    }
    sauces_to_meals.entry("Soy Sauce").or_insert(vec!["Sushi", "Dumplings"]);
    println!("{:?}", sauces_to_meals);

}
