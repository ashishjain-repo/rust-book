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
    
    
}
