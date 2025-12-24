use std::fmt::{Debug, Display, Formatter};


trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) -> String {
        format!("{}", self.get_data())
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self {kind, milk, ounces}
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("*** Coffee! ***")
        .field("☕", &self.kind).field("🥛", &self.milk)
        .field("ounces", &self.ounces).finish()

    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }
    fn get_data(&self) -> String {
        format!("A delicious {} ounce {}", self.ounces, self.kind)
    }
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavour: String,
    percentage: u32,
}

impl Soda {
    fn new(calories: u32, price: f64, flavour: String) -> Self {
        Soda {
            calories,
            price,
            flavour,
            percentage: 100
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }
    fn get_data(&self) -> String {
        format!("Flavour: {}, Calories: {}", self.flavour, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "** {} Soda **", self.flavour)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavour: self.flavour.clone(),
            percentage: self.percentage,
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}

fn main() {
    let mut latte = Coffee::new("Latte", Milk::Whole, 4);
    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappucino = Coffee::new(String::from("Cappuchino"), Milk::Almond, 2);
    println!("Cappuchino data: {}", cappucino.get_data());

    let pepsi = Soda::new(250, 20.0, "Cherry".to_string());
    println!("{}", pepsi);

    let mut coke = pepsi.clone();

    println!("Pepsi == Coke: {}", (pepsi == coke));

    coke.consume();

    println!("Coke: {:?}", coke);
}