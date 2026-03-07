use std::{fmt::format, result};

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // We rewrote the same logic in shorter way where we are not handlelling None and only looking for value if it exist.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    #[derive(Debug)]
    enum UsState {
        Alaska,
    }
    enum Coin {
        Quarter(UsState),
        Penny,
    }

    // This is the pattern matching with match pattern
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}"),
        _ => count += 1,
    }

    // This is the same but with less code and more readablility
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {state:?}");
    } else {
        count += 1;
    }

    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
        }
        enum Coin {
            Quarter(UsState),
            Penny,
        }

        impl UsState {
            fn existed_in(&self, year: u16) -> bool {
                match self {
                    UsState::Alabama => year >= 1819,
                    UsState::Alaska => year >= 1819,
                }
            }

            /* fn describe_state_quarter(coin: Coin) -> Option<String> {
                if let Coin::Quarter(state) = coin {
                    if state.existed_in(1900) {
                        Some(format!("{state:?} is pretty old for America"))
                    } else {
                        Some(format!("{state:?} is relatively new."))
                    }
                } else {
                    None
                }
            } */
            /* fn describe_state_quarter(coin: Coin) -> Option<String> {
                let state = if let Coin::Quarter(state) = coin {
                    state
                } else {
                    return None;
                };
                if state.existed_in(1900) {
                    Some(format!("{state:?} is pretty old, for America!"))
                } else {
                    Some(format!("{state:?} is relatively new."))
                }
            } */

            fn describe_state_quarter(coin: Coin) -> Option<String> {
                let Coin::Quarter(state) = coin else {
                    return None;
                };
                if state.existed_in(1900) {
                    Some(format!("{state:?} is pretty old, for America!"))
                } else {
                    Some(format!("{state:?} is relatively new."))
                }
            }
        }
        let coin: Coin = Coin::Quarter(UsState::Alabama);
        let result = UsState::describe_state_quarter(coin);
        if let Some(message) = result {
            println!("{message}");
        } else {
            println!("maybe none")
        }
    
    }
}
