enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            // we can use curly braces for more logic if we need to.
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ..
}

// We can also add state into our matches + enums
enum StatefulCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_again(coin: StatefulCoin) -> u8 {
    match coin {
        StatefulCoin::Penny => 1,
        StatefulCoin::Nickel => 5,
        StatefulCoin::Dime => 10,
        StatefulCoin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let state = UsState::Alaska;
    let coin = StatefulCoin::Quarter(state);
    println!("{}", value_in_cents_again(coin));
}
