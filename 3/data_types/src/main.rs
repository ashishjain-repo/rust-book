use std::io;

fn main() {
    // There are two types of data types: scalar and compound

    let guess: u32 = "42".parse().expect("Not a number");

    // Scalar Types: Integers, Floating Point Numbers, Booleans, and Characters.
    // Int Types: Signed(+/-) start with i and unsigned(+) start with u and go in this sequence which is same as the bit size they use: 8, 16, 32, 64, 128, and size (Architecture Dependent). Signed store the number from -(2^n-1) to 2^n-1 and unsigned store number from 0 to 2^n - 1. N here represents the bit allocated.

    // Floating Point Type: are the same as we saw in but instead of being whole they are numbers that contain decimals and to declare they use 'f' and the bit size 8, 16, 32, 64, 128
    let x = 2.0;
    let y: f32 = 3.0;

    // Numeric operations:
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.3;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    // Boolean Type: This only stores the values true and false and we can use bool keyword to define the type of the variable
    let t = true;
    let f: bool = false;

    // Character Type: To have charcter variables we define them using keyword char and they are always in between single quotes.
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    // Compound Types: These are the types that can have group of values into one type. There are two primitive compound types: tuples and arrays.

    // Tuples can have multiple comma seperated values with different types if declared that way. Once the tuple size is declared it is fixed, it cannot grow or shrink. We can also get the elements individually out using variables surrounded with paranthese and equallity operator following the tuple (Destructuring).
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of a is {a}, b is {b}, and c is {c}");
    // We can also access the tuple value by using the period "." followed by the index of the value we want to acces.
    println!("Value at index 1 is {} and at index 3 is {}", tup.0, tup.2);

    // Array: This data strucutre is used with following syntax and has fixed length, it cannot grow. Instead of going on a heap it goes on the stack. We can use vectors but when we know the amount we can use the arrays. We can define the type and the size of the array. We access the array element using [] with the index passed into it.
    let a = [1,2,3,4,5,6];
    println!("{}", a[0]);
    let months = ["Jan", "Feb", "March"];
    println!("{} {} {}", months[0], months[1], months[2]);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[0]);

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
