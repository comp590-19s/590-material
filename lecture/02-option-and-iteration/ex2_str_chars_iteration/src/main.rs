fn main() {
    // Declare a string variable
    let a_str = "ab";

    // Establish an iterator to its characters
    let mut itr = a_str.chars();

    if let Some(c) = itr.next() {
        println!("a: {}", c);
    }

    if let Some(c) = itr.next() {
        println!("b: {}", c);
    }

    if let Some(c) = itr.next() {
        println!("c: {}", c);
    } else {
        println!("no c!");
    }
}
