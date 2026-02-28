fn main() {
    // To work with references instead of taking ownership of the value we pass them by reference using '&' when passing and in the function as well. This will only pass the reference and does not transfer the ownership. Since it is not owned the value is not dropped when it is out of the scope. This can also be called as borrowing because we are not taking ownereship instead we are just borowwing the reference to it and then we are not obliged to give back the ownership since we never had it.
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{s1}' is {len}");
        fn calculate_length(s: &String) -> usize {
            s.len()
        }
    }
    // Here we are borrowing but also we are trying to manipulate the string like we own but this results into a compilation error because the reference object cannot be manipulated.
    /* {
        let s = String::from("Hello");
        change(&s);
        fn change(s: &String) {
            s.push_str("world");
        }
    } */
    // Mutable References: We can pass the references as mutable by changing the signature to &mut and then we will be able to change the borrowed value. But the catch is we cannot have more mutable references/borrow to the same object.
    {
        let mut s: String = String::from("hello");
        change(&mut s);
        println!("{s}");
        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }
    }
    // This code will attempt to make two mutable references to that same object first one works but second results into a complie time error.
    /* {
        let mut s: String = String::from("Hello");
        let r1 = &mut s;
        let r2: = &mut s;
        println!("{r1}, {r2}");
    } */
    // This is a saftey net so we do not manipulate the same reference different times in the same scope. But if we go out of the scope rust permits us to do that because that is valid, because the first referenece is out of scope so we can now create the second reference.
    {
        let mut s: String = String::from("hello");
        {
            let r1 = &mut s;
            println!(
                "r1 is here {}, but will go out of scope after curly brackets",
                r1
            );
        }
        let r2 = &mut s;
        println!("New reference r2: {} created", r2);
    }
    // Rust has the similar rule when combining the mutable and immutable references, if we keep passing the reference to multiple object that is fine but the moment we drop a &mut in between that will cause an error. This is there because the users of the immutable object don;t want the reference to be changed.
    /* {
        let mut s: String = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        let r3 = &mut s;
        println!("r1 {}, r2 {}, r3 {}",r1, r2, r3);
    } */
    // This is somewhat similar but the r1 and r2 are not being used later so there are no issue because they are not chagned, but if they are changed or passed by reference with mutability it means they could then it will result in an error.
    {
        let mut s: String = String::from("Hello");
        let r1 = &s;
        let r2 = &s;
        println!("r1: {}, r2: {}", r1, r2);

        let r3 = &mut s;
        println!("r3: {}", r3);
    }

    // Dangling References
    // Here we are referencing or function is trying to return some reference but the compiler tells us that there is no reference because the moment s inside the function goes out of the scope which is after the function is finished there is not s in the memory so we are trying to assign the reference when there is not reference to that string.
    /* {
        let reference_to_nothing = dangle();
        fn dangle() -> &String {
            let s = String::from("hello");
            &s
        }
    } */
}
