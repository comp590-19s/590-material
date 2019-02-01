fn main() {
    // Declare a 2-element vec and array
    let a: Vec<u32> = vec![110, 110];
    let b: [u32; 2] = [590, 590];

    // Print the address of each value
    println!("&a:    {:p}", &a);
    println!("&b:    {:p}", &b);

    // Print the address of first elems
    println!("&a[0]: {:p}", &a[0]);
    println!("&a[1]: {:p}", &a[1]);
    println!("&b[0]: {:p}", &b[0]);
    println!("&b[1]: {:p}", &b[1]);
}
