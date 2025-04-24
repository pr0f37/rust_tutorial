use std::{collections::LinkedList, io};

fn main() {
    let mut numbers: LinkedList<u64> = LinkedList::new();
    loop {
        println!("\nGive me a number or a command 'get' or 'exit'");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim();
        match trimmed_input {
            "get" => get_median(&numbers),
            "exit" => break,
            _ => {
                match trimmed_input.parse::<u64>() {
                    Ok(number) => add_number(&number, &mut numbers),
                    Err(_) => {
                        println!("It's neither 'get', 'exit' nor a number!");
                        continue;
                    }
                };
            }
        }
    }
}

fn add_number(number: &u64, numbers: &mut LinkedList<u64>) {
    numbers.push_back(*number);
}

fn get_median(numbers: &LinkedList<u64>) {
    println!("get_median of {:?}", numbers);
}
