fn main() {
    let mut trip = start_trip();
    goa(&mut trip);
    pondy(&mut trip);
    amsterdam(&mut trip);
    println!("{trip}");
}

fn start_trip() -> String {
    return String::from("Trip plan is...");
}
fn goa(tripStr: &mut String) {
    tripStr.push_str(" Goa ");
}

fn pondy(tripStr: &mut String) {
    tripStr.push_str(" Thailand ");
}

fn amsterdam(tripStr: &mut String) {
    tripStr.push_str(" Amsterdam ");
}