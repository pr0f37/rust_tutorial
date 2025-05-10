use std::io;

fn main() {
    let a = get_number();
    let operator = get_operator();
    let b = get_number();

    println!("Result:");
    println!("{}", operator(a, b));
}

fn get_number() -> f32 {
    println!("Please enter a number:");
    loop {
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Failed to read line");
        match a.trim().parse::<f32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Error. Please reenter a number");
            }
        };
    }
}

fn get_operator() -> fn(f32, f32) -> f32 {
    println!("Please enter the operator:");
    loop {
        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("Failed");
        match operator.trim() {
            "+" => return sum,
            "-" => return difference,
            "/" => return division,
            "*" => return multi,
            _ => {
                println!("Error. Please reenter one of the following operators +,-,*,/");
            }
        };
    }
}

fn sum(a: f32, b: f32) -> f32 {
    a + b
}

fn difference(a: f32, b: f32) -> f32 {
    a - b
}

fn division(a: f32, b: f32) -> f32 {
    a / b
}

fn multi(a: f32, b: f32) -> f32 {
    a * b
}
