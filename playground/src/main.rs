fn main() {
    let hw = String::from("Hello World");
    let hi = String::from("hi");

    println!("{}", first_word(&hw));
    println!("{}", first_word(&hi));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
