#[derive(Debug)]
struct Food {
    name: String
}
#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return Option::None
        }
        if self.reservations < 12 {
            return Option::Some(Food{name:"Uni Sashimi".to_string()})
        }
        Option::Some(Food{name:"Strip Steak".to_string()})
    }
    
    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Result::Err("Sorry we have a mice problem".to_string());
        }
        if address.is_empty() {
            return Result::Err("User has not specified address".to_string());
        }
        return Result::Ok(Food {name: String::from("Burger")});
    }
}

fn main() {
    let res1 = Restaurant {
        reservations: 11,
        has_mice_infestation: true
    };
    println!("Chef special is {:?}", res1.chef_special());
    println!("Burger being delivered : {:?}", res1.deliver_burger("123 Elm street"));

    let res2 = Restaurant {
        reservations: 15,
        has_mice_infestation: false
    };

    println!("Chef special is {:?}", res2.chef_special());
    println!("Burger being delivered : {:?}", res2.deliver_burger(""));
    println!("Burger being delivered : {:?}", res2.deliver_burger("123 Elm street"));
    
}
