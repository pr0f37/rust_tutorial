fn main() {
    let x: i32 = 5;
    println!("the value of x is: {x}");
    let x = x + 1;
    println!("the value of x is: {x}");
    {
        let x = x + 2;
        println!("the value of x is: {x}");
    }
    println!("the value of x is: {x}");
}
