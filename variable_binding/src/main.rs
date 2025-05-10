fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;
    println!("An intgeger: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("An unit value {:?}", unit);

    let _unsused_variable = 3u32;

    let noisy_unused_variable = 4u32;

    let immutable_variable = 1;

    let mut mutable_variable = 2;
    mutable_variable += 1;
    println!("Mutable variable: {:?}", mutable_variable);

    // shadowing occurs when variable declared in inner block has the same name as the one in outer
    // block and is being set on top of the one in outer block unntil the inner block finish

    let shadow_binding = 1;

    {
        println!("Before being shadowed: {}", shadow_binding);
        let shadow_binding = "abc";
        println!("Shadowed in inner block: {}", shadow_binding);
    }
    println!("Out of shadowed block: {}", shadow_binding);

    let shadow_binding = 2;
    println!("shadowed in outerblock: {}", shadow_binding);
}
