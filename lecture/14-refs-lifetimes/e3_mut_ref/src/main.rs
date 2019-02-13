fn main() {
    let a = 0;
    let a_mut_ptr = &mut a;
    *a_mut_ptr = 1;

    let mut y = 10;
    let mut z = 20;
    let yz_mut_ptr = &mut y;
    *yz_mut_ptr = 99;
    yz_mut_ptr = &mut z;
    *yz_mut_ptr = 99;

    println!("{} {} {}", &a, &y, &z);
}
