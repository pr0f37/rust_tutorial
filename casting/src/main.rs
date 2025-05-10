#![allow(overflowing_literals)]
use std::mem::size_of_val;

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    #[warn(unused_variables)]
    let decimal = 60.4321_f32;

    // let integer: u8 = decimal;
    let integer: u8 = decimal as u8;
    let character: char = integer as char;

    // let character: char = decimal as char;
    println!("Casting: {}->{}->{}", decimal, integer, character);
    println!("1000 as u16 int: {}", 1000 as u16);
    println!("1000 as u8 int: {}", 1000 as u8);
    println!("-1 as u8 int: {}", (-1i8) as u8);
    println!("1000 mod 256: {}", 1000 % 256);
    println!("NAN as u8: {}", f32::NAN as u8);
    unsafe {
        println!("300 as u8: {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100 as u8: {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("NAN as u8: {}", f32::NAN.to_int_unchecked::<u8>());
    }

    // suffixed literals, their types are known at initialization;

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", size_of_val(&x));
    println!("size of `y` in bytes: {}", size_of_val(&y));
    println!("size of `z` in bytes: {}", size_of_val(&z));
    println!("size of `i` in bytes: {}", size_of_val(&i));
    println!("size of `f` in bytes: {}", size_of_val(&f));

    // type inference
    let elem = 5u8;

    let mut vec = Vec::new();

    vec.push(elem);
    println!("{:?}", vec);

    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
