#[derive(Debug)]
enum Tier {
    GOLD,
    SILVER,
    PLATINUM,
}
#[derive(Debug)]
enum Subscription {
    FREE,
    BASIC(f64,u32),
    PREMIUM {tier: Tier},
}
impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::FREE => println!("You have limited access to the site"),
            Subscription::BASIC(price,months) => println!("You have limited access to Premium features at {price} for {months} months"),
            Subscription::PREMIUM { tier } => println!("You have full access to premium features. Your tier is {tier:?}"),
        }
    }
}
fn main() {
    let free_sub = Subscription::FREE;
    free_sub.summarize();

    let basic_sub = Subscription::BASIC(1000.0, 6);
    basic_sub.summarize();

    let prem_sub = Subscription::PREMIUM { tier: Tier::PLATINUM };
    prem_sub.summarize();
}
