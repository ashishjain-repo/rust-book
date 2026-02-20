fn main() {
    let x = 5;
    println!("The value of x is {x}");
    /* This whole section is erroronous because we have created a immutable variable and assigning a new value like this will break the program because it violates the immutability laws.
    x = 6;
    println!("The value of x is {x}");
    */

    // This section wont cause any error because we have declared a mutable variable named y with a default type that has been assinged to it based on the value that we have entered on the initialization.
    let mut y = 10;
    println!("The value of y is {y}");
    y = 11;
    println!("The value of y is {y}");

    // Constants can be delcared using const keyword and they do not accept mut because they are always immutable by default. As a convention constant are declared using all capital and underscore to break words
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    // Shadowing - We can shadow the same variable by declaring it again so for the sake of the program the compiler will detect that new value of that variable unless it gets out of the scope then the previous value will be recognized

    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}");

    // This code is perfectly fine because rust is treating it as a new variable instead of the existing variable.
    let spaces = "   ";
    let spaces = spaces.len();

    /* This section on the other hand is error, even though we have a mutable variable but rust complier won't allow us to change the type of the variable which will return in a type error during compile time.
    let mut spaces2 = "   ";
    spaces2 = spaces2.len(); 
    */

}
