#[derive(Debug)]
struct SuperMarketItem {
    name: String,
    price: f64,
}
#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SuperMarketItem>,
}
impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F) where F: FnMut(&mut SuperMarketItem)  {
        let mut i = 0;
        while i < self.items.len() {
            operation(&mut self.items[i]);
            i += 1;
        }
    }
    fn checkout<F>(self, operation: F) where F: FnOnce(ShoppingCart) {
        operation(self);
    }
}
fn main() {
    let mut cart = ShoppingCart {
        items: vec![SuperMarketItem{name:"Apple".to_string(), price:10.95}, SuperMarketItem{name:"Banana".to_string(), price: 5.65}]
    };
    cart.traverse_items(|sitem| {sitem.price = sitem.price*0.85});
    println!("{:?}",cart);
    cart.traverse_items(|sitem| {sitem.name = sitem.name.to_lowercase()});
    println!("{:?}",cart);
    let mut total_price = 0.0;
    cart.checkout(|mut c| {
        println!("{:?}",c);
        c.traverse_items(|sitem| {total_price += sitem.price});
    });
    println!("Total price : ${:.2}", total_price);
}
