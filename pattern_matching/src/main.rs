// Pattern Matching
// Pattern matching in Rust allows you to destructure enums, structs, and other types in a concise way.
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    skadoosh,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {  
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
        Coin::skadoosh => 19
    }
}

fn main() {
    let res = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("res is {res}");
}
