use std::fmt::format;

fn main() {
    // There are two types of Strings there is String and then there is string slice str.
    // This string is actually a wrapper around Vector, with some restrictions.
    {
        let mut s = String::new(); // Creates a new instance of string
        // to_string method apply the Display trait to a string literal directly or on a variable
        let data = "initial contents";
        let s = data.to_string();
        let s = "inital contents".to_string();

        let s = String::from("initial contents");
    }
    {
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שלום");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }
    // We can use the push_str method on the mutable string to add more data.
    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{}", s);
    }
    // Here we did not lose the ownership of the s2 and s2 is accessible even after passing it in push_str function.
    {
        let mut s: String = String::from("foo");
        let s2 = "bar";
        s.push_str(s2);
        println!("s2 is {s2} and s is {s}");
    }

    // Combining two existing strings using + operator. We are using reference for s2 because we lost the ownership of s1 but the add method we are using has a signature of referencec instead of actual ownership that is why we are using reference.
    {
        let s1: String = String::from("Hello, ");
        let s2: String = String::from("World!");
        let s3: String = s1 + &s2;
    }
    // Format macro to format string instead of using + operator to concatenate strings. format! macro is like println! but instead of printing it returns the string. And by using this we are not transfering ownership because format! uses references.
    {
        let s1: String = String::from("tic");
        let s2: String = String::from("tac");
        let s3: String = String::from("toe");
        let s: String = format!("{s1}-{s2}-{s3}");
        println!("s is {s}");
    }
    // Iterating over a string can be done in two ways, one by using the chars methods to get individual character, or bytes method to get the raw byte those characters are stored in.
    {
        for c in "Зд".chars() {
            println!("{c}");
        }
        for b in "Зд".bytes() {
            println!("{b}");
        }
    }
}
