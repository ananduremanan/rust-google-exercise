// Recurssion with if else block
fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

// While Loop Example
fn while_loop() {
    let mut x = 20;
    while x > 1 {
        x = x / 2;
        println!("X is : {}", x);
    }
}

fn main() {
    let n: u32 = 1;
    println!("fib({n}) = {}", fib(n));
    while_loop()
}
