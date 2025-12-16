fn colour_to_number(colour: &str) -> i32 {
    return match colour {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial_iterative(mut num: i32) -> i32 {
    let mut res = 1;
    while num > 1 {
        res *= num;
        num -= 1;
    }
    return res;
}

fn factorial_recursive(num: i32) -> i32 {
    if num == 1 {
        return num;
    }
    return num*factorial_recursive(num-1);
}

fn main() {
    println!("Red is {}", colour_to_number("red"));
    println!("Yellow is {}", colour_to_number("yellow"));
    println!("Factorial of 5 is {}", factorial_iterative(5));
    println!("Factorial of 7 is {}", factorial_recursive(7));
}
