fn main() {
    for n in 1..100 {
        let mut tmp: String = "".to_string();
        if n % 3 == 0 {
            tmp.push_str("fizz");
        }
        if n % 5 == 0 {
            tmp.push_str("buzz");
        }
        if tmp == "" {
            println!("{}", n);
        } else {
            println!("{}", tmp);
        }
    }
}
