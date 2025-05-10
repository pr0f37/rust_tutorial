fn main() {
    let mut x: String = String::from("Hello!");
    println!("main: {x}");
    take_ownership(&mut x);
    println!("main: {x}");
    destroy_objects();
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

fn destroy_objects() {
    let s1: String = String::from("t");
    let s2: String = s1.clone();
    println!("{s1}");
    println!("{s2}");
}
