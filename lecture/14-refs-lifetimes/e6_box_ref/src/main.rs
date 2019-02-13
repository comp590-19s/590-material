fn main() {
    let mut a: u32 = 0;
    let b: Box<&mut u32> = Box::new(&mut a);
    **b += 1;
    println!("{}", a);
}


