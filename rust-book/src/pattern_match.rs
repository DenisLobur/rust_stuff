pub(crate) enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
    // --snip--
}


pub fn value_in_cents(coin: Coin) -> (u8, UsState) {
    match coin {
        Coin::Penny => (1, UsState::Alabama),
        Coin::Nickel => (5, UsState::Alaska),
        Coin::Dime => (10, UsState::Alabama),
        Coin::Quarter(state) => (25, state)
    }
}