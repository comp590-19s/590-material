fn main() {
    let s = foo(String::from("a"));
    println!("{}", s);
}

fn foo(s: String) -> String {
    bar(&s)
}

fn bar(s: &str) -> String {
    println!("{}", s);
    return String::from("c");
}


