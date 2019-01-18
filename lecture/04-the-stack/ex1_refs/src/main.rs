fn main() {
    let a: u8 = 0;
    print_address("&a", &a);

    let b: u8 = 1;
    print_address("&b", &b);

    let p: &u8 = &b;
    print_address("p", p);

    println!("{}", *p);
}

fn print_address(label: &str, address: &u8) {
    println!("{:p} - {}", address, label);
}
