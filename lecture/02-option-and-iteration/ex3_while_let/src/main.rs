fn main() {
    let a_str = "abcde";

    let mut itr = a_str.chars();

    while let Some(c) = itr.next() {
        match c {
            'a' => {
                println!("a");
            }
            'e' => {
                println!("e");
            }
            _ => {
                println!("not a nor e");
            }
        }
    }
}
