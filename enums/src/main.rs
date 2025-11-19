enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("coin from {state}");
            25
        }
    }
}

fn main() {
    let one = Coin::Penny;
    let two = Coin::Quarter(String::from("california"));

    println!("value of one: {}", value_in_cents(one));
    println!("value of two: {}", value_in_cents(two));
}
