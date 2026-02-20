fn main() {
    {
        let number = 3;
        // If the condition is not bool rust will not try to convert into a bool, and will throw an error.
        if number < 5 {
            println!("Condition was true");
        } else {
            println!("Condition was false");
        }
    }

    {
        let number = 6;
        // using if else if ladder
        if number % 4 == 0 {
            println!("Number is divisible by 4");
        } else if number % 3 == 0 {
            println!("Number is divisible by 3");
        } else {
            println!("Number is not divisible by 3 or 4");
        }
    }

    {
        // using the if in the statement and assigning the value based on the codition being true or false.
        let condition = true;
        let number = if condition { 5 } else { 6 };
        println!("The value of number is: {number}");
    }

    {
        // This will throw an error because there is a mismatch between the return types one return i32 and the other returns a string, so both values has to be the same type
        /* let condition = true;
        let number = if condition {5} else {"Six"};
        println!("The value of number is: {number}"); */
    }
    // Rust has three kind of loops: while, for, and loop
    {
        // loop keyword creates a loop that runs forever until explcilty been told to stop.
        /* loop { // This program will run forever and can be terminated using Ctrl+C but we will comment it out, and is not a good practice there should always be a condition to terminate a loop
            println!("again");
        } */
    }
    {
        // Returning values from within the loop, and using the break keyword to break out of the infinte loop when certain condition is met
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is result: {result}");
    }
    {
        // We can lable the loops so we can use those lables inside the inner loops where we use break or continue. These lables must start with "'"
        // In this small program we have loop inside loop inside loop breaks when the remainng value is 9 and move to the outer loop where the count is being increased, but if the count is 2 the outer loop break by using the label that we have for outer loop and then we exit the program by printing the end count. We can see that remaining keep getting back to 10 because the remaining is defined in the scope of first loop and whenever we exit the inner loop it is reinitialized with the value of 10.
        let mut count = 0;
        'counting_loop: loop {
            println!("count: {count}");
            let mut remaining = 10;
            loop {
                println!("remaining: {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_loop;
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!("End count = {count}");
    }

    {
        // Conditional Loop with While Loop
        let mut number = 3;
        while number != 0 {
            println!("Number: {number}");
            number -= 1;
        }
        println!("LIFTOFF!!!");
    }

    {
        // Looping through a collection with for
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
        while index < 5 {
            println!("The value is: {}", a[index]);
            index += 1;
        }

        // Same thing but iterating over the array using for loop
        for element in a {
            println!("The value is: {element}");
        }
    }

    {
        // here is an example of using range function when interating with a rev function to reverse that array
        for number in (1..4).rev() {
            println!("{number}");
        }
        println!("LIFTOFF!!!");
    }
}
