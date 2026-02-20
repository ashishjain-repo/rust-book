fn main() {
    println!("Hello, world!");
    another_function();
    another_function_parameter(5);
    another_function_mul_para(22, String::from("This is string"));
    expression();
    let mut x = five();
    println!("The value of x: {x}");
    x = plus_one(x);
    println!("The value of x: {x}");
}

// To declare a function we use fn keyword then the name of the function snake_case as convention in rust, and it does not matter that we have declared it after main, what matters is that the function is in the scope.
fn another_function() {
    println!("Another function.");
}

fn another_function_parameter(x: i32) {
    println!("The value of x is: {x}");
}

// Function with multiple parameters
fn another_function_mul_para(x: i32, y: String) {
    println!("The value of x: {x}, and y: {y}");
}

// We cannot to the following because rust statements does not return anything. Statments does not support statement, declaring a new value is a statement not an expression.
/* fn statements() {
    let y = (let x = 10);
} */

// This will not be consider statement in a statment because at the end it is an experssion because we have a statemente that says x = 3 but then we are returning an expression x + 1 which is an experssion. Again statements do not return a value.
fn expression() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}


// Functions with return values. We do not name the return value but we do declare the type we are returning after the arrow '->'. The final expression becomes the return value but if we want to retun something early we can use the return keyword for that.
fn five() -> i32 {
    5
}

// In this function we are taking an argument and returning an expression, but if we add a semicolon to this expressio we will turn it into a statement. But if we add the return keyword to it explicity and leave the semicolon it will work, this error occured because of the type mismatch
/* fn plus_one(x: i32) -> i32 {
    x + 1;
} */
fn plus_one(x: i32) -> i32 {
    return  x + 1;
}