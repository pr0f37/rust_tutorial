use std::{cmp::Ordering, io, vec::Vec};

fn main() {
    let mut numbers: Vec<u64> = Vec::new();
    // let mut numbers: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 23, 23];
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
                    Ok(number) => add_number(number, &mut numbers),
                    Err(_) => {
                        println!("It's neither 'get', 'exit' nor a number!");
                        continue;
                    }
                };
            }
        }
    }
}

fn add_number(number: u64, numbers: &mut Vec<u64>) {
    if numbers.is_empty() {
        numbers.push(number)
    } else {
        let index = get_index(&number, numbers);
        numbers.insert(index, number);
    };
}

fn get_median(numbers: &Vec<u64>) {
    println!("get_median of {:?}", numbers);
    if !numbers.is_empty() {
        let index = numbers.len() / 2;
        if numbers.len() % 2 != 0 {
            println!("{}", numbers[index]);
        } else {
            println!("{}", ((numbers[index - 1] + numbers[index]) as f32 / 2.0));
        }
    }
}

fn get_index(number: &u64, numbers: &mut Vec<u64>) -> usize {
    if numbers.len() == 1 {
        if *number < numbers[0] {
            return 0;
        }
        return 1;
    };
    let mut h_index = numbers.len() - 1;
    let mut l_index = 0;
    // [1,3,5,7,9]
    loop {
        let tmp_index = (h_index + l_index) / 2;
        match (*number).cmp(&numbers[tmp_index]) {
            Ordering::Equal => return tmp_index,
            Ordering::Less => {
                if h_index <= l_index || tmp_index == 0 {
                    return tmp_index;
                }
                h_index = tmp_index - 1;
            }
            Ordering::Greater => {
                if h_index <= l_index {
                    return tmp_index + 1;
                }
                l_index = tmp_index + 1;
            }
        };
    }
}
