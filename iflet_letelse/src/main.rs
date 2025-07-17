#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska  => year >= 1959
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> { //TODO understand this
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn discribe_state_quarter_ifbase(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    }; //Using if let return value
    
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is realtively new."))
    }
}

fn describe_state_quarter_letelse(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    }; //This ways we save the state branch, we need only the none. let...else
    
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is realtively new."))
    }
}

fn main() {
    let config_max: Option<u8> = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configure to be {max}"),
        _ => ()
    }
    
    // Alternative using if let
    // Usefult to catch only one arm, and igore all the others
    if let Some(max) = config_max { // This is one are of a match expression
        println!("The maximum is configured to be {max}");
    } else {
        println!("The value is non-exising");
    }
    
    //Staying on the Happy path with let...else
    //Commong thing: Do a calculation if the value is exiting, return a default if not
    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alaska)) {
        println!("{desc}");
    }
    
    if let Some(desc) = discribe_state_quarter_ifbase(Coin::Quarter(UsState::Alaska)) {
        println!("{desc}");
    }
    
    if let Some(desc) = describe_state_quarter_letelse(Coin::Quarter(UsState::Alaska)) {
        println!("{desc}");
    }
    
    
    /*
    
       KEY TAKEAWAY
       If you have a situation in which your program has logic that is
       too verbose to express using a match, remember that
       if let and let...else are in your Rust toolbox as well.
    
    */
    
}
