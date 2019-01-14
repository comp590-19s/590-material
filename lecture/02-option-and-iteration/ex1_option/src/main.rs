fn main() {
    let a: Option<char> = Some('a');
    let b: Option<char> = None;

    if let Some(c) = a {
        println!("{}", c);
    }

    if let Some(c) = b {
        println!("{}", c);
    } else {
        println!("None");
    }

    if let None = b {
        println!("None");
    }
}
