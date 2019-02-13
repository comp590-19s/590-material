fn main() {
    let word = String::from("hi");
    let word3 = repeat(word, 3);
    println!("word: {}, word3: {}", &word, &word3);
}

fn repeat(word: String, i: usize) -> String {
    if i <= 0 {
        String::from(word)
    } else {
        let mut repeated = repeat(word, i - 1);
        repeated.push_str(&word);
        repeated
    }
}


