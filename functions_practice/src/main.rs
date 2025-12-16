fn main() {
    apply_to_jobs(5, "System architect");
    println!("2 is {}", is_even(2));
    let mut az_tuple = contains_az("aardvard");
    println!("aardvark => a={}, z={}", az_tuple.0, az_tuple.1);
    az_tuple = contains_az("zoology");
    println!("zoology => a={}, z={}", az_tuple.0, az_tuple.1);
    az_tuple = contains_az("zebra");
    println!("zebra => a={}, z={}", az_tuple.0, az_tuple.1);
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I am applying to {number} {title} jobs");
}

fn is_even(number: i32) -> bool {
    number%2==0
}

fn contains_az(string: &str) -> (bool, bool) {
    (string.contains('a'), string.contains('z'))
}
