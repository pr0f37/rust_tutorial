fn main() {
    let mut x: String = String::from("Hello!");
    println!("main: {x}");
    take_ownership(&mut x);
    println!("main: {x}");
}

fn take_ownership(x: &mut String) {
    x.push_str(" take ownership");
    wrapped_take_ownership(x);
    println!("take_ownership: {x}");
}

fn wrapped_take_ownership(x: &mut String) {
    x.push_str(" wrapped_take_ownership");
    println!("wrapped_take_ownership: {x}");
}
