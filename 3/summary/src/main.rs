fn main() {
    println!("Hello, world!");
    let c = f_to_c(100.0);
    let f = c_to_f(37.77778);
    println!("100 F to C is: {c}");
    println!("37.77778 C to F is: {f}");
    let fib = fibonacci(40);
    println!("Fibonacci number of 40 is: {fib}");
}

fn f_to_c(f: f32) -> f32{
    let c: f32 = (f - 32.0) * 5.0/9.0;
    return c;
}
fn c_to_f(c: f32) -> f32 {
    let f: f32 = (c * 1.8) + 32.0;
    return f;
}

fn fibonacci(n: u128) -> u128{
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}