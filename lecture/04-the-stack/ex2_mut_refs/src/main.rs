fn main() {
    let mut a: u8 = 0;
    print_address("&a", &a);

    let mut b: u8 = 1;
    print_address("&b", &b);

    let mut p: &mut u8 = &mut a;
    print_address("p", p);
    *p = 10;

    p = &mut b;
    print_address("p", p);
    *p = 11;

    println!("a:{} - b:{}", a, b);
}

fn print_address(label: &str, value: &u8) {
    println!("{:p} - {}", value, label);
}
