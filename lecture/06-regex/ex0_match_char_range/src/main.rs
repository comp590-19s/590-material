fn main() {
    let input = "abcDEfghi;123";
    println!("input: {}", input);

    let mut some_chars = input.chars();
    while let Some(c) = some_chars.next() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                println!("vowel: {}", c);
            }
            'A'...'Z' => {
                println!("capital: {}", c);
            }
            'a'...'z' => {
                println!("lowercase: {}", c);
            }
            _ => {
                println!("other: {}", c);
            }
        }
    }
}
