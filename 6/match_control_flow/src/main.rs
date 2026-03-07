fn main() {
    // Match is like a case or switch statement, it consists of arms tthat has two parts pattern and some code. It is not exactly like switch case statement where one variable have values and switch try's to match it but in match it matches the instance we have created, so if the instance is of one of the Enum values then it matches that.
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
    }

    // Patterns that Bind to values
    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            South_Dakota,
            California,
        }
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {state:?}");
                    25
                }
            }
        }
        // To use this function we would do somthing like this:-
        let my_coin = Coin::Quarter(UsState::Alabama);
        let value = value_in_cents(my_coin);
        println!("The value is {} cents", value);
    }

    // The Option<T> match pattern
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("Value with input 5 '{:?}', and with None '{:?}'", six, none);
    }

    // Matches are exhaustive
    {
        // This will result in an error because match wants you to cover all the cases. This code require None to be handled if we can add that it will work
        /* fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                Some(i) => Some(i + 1),
            }
        } */
    }

    // Catch-All Patterns and the _ Placeholder
    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {}
    }

    // We can just have the placeholder '_' which can be other match all pattern which satisfies the requirement of match for having all the possible values handled, in this case we are either rerolling on not doing anything, this is permited because we are not using that value at all.
    {
        {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            // _ => reroll(),
            _ => (),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {}
    }
    }
}
