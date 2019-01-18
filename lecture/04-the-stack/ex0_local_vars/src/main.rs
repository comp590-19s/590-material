fn main() {
    let a: u8 = 1;
    print_address("main a", &a);
    a_fun(a);
}

fn a_fun(a: u8) {
    let b: u8 = 2;
    let c: u8 = 3;
    print_address("a_fun a", &a);
    print_address("a_fun b", &b);
    print_address("a_fun c", &c);
    println!("{}", a + b + c);
}

fn print_address(label: &str, value: &u8) {
    println!("{:p} - {}", value, label);
}
