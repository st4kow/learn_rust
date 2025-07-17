enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1  // NO ;! This is the return value
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
    //Pattern that bind to Values
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1) //Implicitly created i
    }  //Returns the match output, because there is no ; at the end
}

fn main() {
    println!("Hello, world!");
    let coin: Coin = Coin::Quarter(UsState::Alaska);
    println!("Value of the Quarter: {}", value_in_cents(coin));
    
    //Using match for Option<T>s
    let five: Option<i32> = Some(5);
    let six:  Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);
    
    //Catch-All pattens and the _ Placeholder
    
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other) //This catches everything
    }
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll() //This also catches everything, but not binds to value
    }
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => () //Unit tuple in other cases. Do nothing
    }
    
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
}
