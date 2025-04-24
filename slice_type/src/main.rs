fn main() {
    let s: String = String::from("Hello world");

    let word: &str = first_word(&s);
    println!("{word}");

    let s2: &str = "Bye world";

    let word = first_word(s2);

    println!("{word}");
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
