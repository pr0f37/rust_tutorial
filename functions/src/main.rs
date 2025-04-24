fn main() {
    let fib: i32 = fibonacci(4);
    println!("The final result is: {fib}");
    let fib_rec: i32 = fibonacci_rec(4);
    println!("The final result is: {fib_rec}");
}

fn fibonacci(x: i32) -> i32 {
    println!("Fibonacci number {x} calculation start!");
    if x == 1 {
        return 0;
    } else if x == 2 {
        return 1;
    }
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    let mut counter: i32 = 2;
    while counter < x {
        let c: i32 = a + b;
        a = b;
        b = c;
        counter += 1;
    }
    println!("Fibonacci number calculated!");
    return b;
}

fn fibonacci_rec(x: i32) -> i32 {
    if x == 1 {
        return 0;
    };
    if x == 2 {
        return 1;
    };
    return fibonacci_rec(x - 1) + fibonacci_rec(x - 2);
}
