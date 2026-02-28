fn main() {
    // How the ownership works with the variable is, they have a scope and when the variable goes out of that scope the value is dropped. These scopes are distinguished by the curly brackets
    // here we see that A string literal is immutable but when we create a string using String type with the from function, this is going to the heap and that is why it is mutable, but string literals are not.
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");
    {
        let s = "hello";
    }
    // With string literals we know the space at the compile time, because they are hard coded, but with string type we have to allocate memory because they can grow.
    // When we do String::from we are requestting the allocation
    // Here we can see that we allocated the memory for the String but when the variable goes out of the scope, GC automatically removes/cleans it from the heap.
    {
        let s = String::from("String");
    }

    // Variables and Data Interacting with Move
    // This creates a copy of x in the variable y. This is pushed to the stack. But this is not the case with Strings
    let x: i32 = 5;
    let y = x;
    // In this case what has happened is that str2 is being refered also by str3 which means they both are references to the same Object in the memory/heap.
    let str2: String = String::from("String");
    let str3: String = str2; // This works but as we talked about cleaning or dropping stuff, but when rust try to drop or free the memeory they will be refering to the same content in the heap, so when both of them goes out of the scope we will run into an error where gargbage collector is trying to remove two things where only one exist, this can lead to security vulnerabilites.
    // What rust does in this case is it free's up the str2 first and make it out of scope and str3 becomes the primary holder to that reference. So in this example instead of making a copy the value or the reference was moved.
    {
        let mut s = String::from("hello");
        s = String::from("ahoy");
        println!("{s}, world");
        // What is happening here is we created a string and then we changed the value now the new value is in the heap and being refered and the old value is now dropped from the memory.
    }

    // Variables and Data Interacting with Clone
    {
        // We use clone method to create deep copies which goes in heap not stack. This is not the case with integers or other variables because they are on the stack so when we assign them to a variable the copy is created.
        let s1: String = String::from("Hello");
        let s2: String = s1.clone();
        println!("s1 = {s1} and s2 = {s2}");
    }
    // Ownerships and Functions
    {
        let s: String = String::from("hello");
        takes_ownership(s);
        // If we try to use s again after being used in the fuction it will thrown a compile time error because the reference has been passed and the memory has been freed so the s does not exist anymore.
        let x: i32 = 5;
        makes_copy(x);
        // We are good to use x still because x made a copy because it has that trait so it is still in the memory.
    }
    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }
    fn makes_copy(some_integer: i32) {
        println!("{some_integer}")
    }

    {
        let s1 = gives_ownership();
        let s2 = String::from("Hello");
        let s3 = takes_and_gives_back(s2);
        fn gives_ownership() -> String {
            let some_string = String::from("yours");
            some_string
        }
        fn takes_and_gives_back(a_string: String) -> String {
            a_string
        }
    }

    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{s2}' is {len}.");

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len(); // len() returns the length of a String
            (s, length)
        }
    }
}
