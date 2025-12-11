fn main() {
    let torrent: i32 = 1_337;
    let small_torrent: i16 = torrent as i16;

    let any_float = 5.5324;
    println!("number is {:.2}", any_float);

    let with_milk = true;
    let with_sugar = false;

    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;

    let small_ints:[i8;4] = [1,2,3,4];
    println!("small ints {:?}", small_ints);
    
    let my_tuple = (56, 3.14, "super duper", small_ints);
    dbg!(my_tuple);
}
