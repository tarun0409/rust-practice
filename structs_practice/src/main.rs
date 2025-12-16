#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}
impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Flight {
        Flight {
            origin, destination, price, passengers,
        }
    }
    fn from_flight(sample_flight: &Flight, origin: String, destination: String) -> Flight {
        Flight::new(origin, destination, sample_flight.price, sample_flight.passengers)
    }
    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }
    fn increase_price(&mut self) {
        self.price = self.price*1.20;
    }
    fn itenerary(&self) {
        println!("({} -> {})", self.origin, self.destination);
    }
}
fn main() {
    let mut my_flight = Flight::new(String::from("Bengaluru"), String::from("Chennai"), 5000.0, 2);
    println!("{:?}", my_flight);
    my_flight.change_destination(String::from("Paris"));
    my_flight.increase_price();
    println!("{:?}", my_flight);
    my_flight.itenerary();
    let second_flight = Flight::from_flight(&my_flight,
        String::from("Mumbai"), String::from("Delhi"));
    println!("{:?}", second_flight);
}
