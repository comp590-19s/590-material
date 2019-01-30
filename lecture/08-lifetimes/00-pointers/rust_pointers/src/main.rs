fn main() {

    let mut a: u16 = 0;
    println!("a: {}", a);

    {
        // Take mutable ref inside block...
        let a_ptr: &mut u16 = &mut a;
        println!("a_ptr: {:p}", a_ptr);
        println!("*a_ptr: {}", *a_ptr);
        *a_ptr = 1;
        // So that after block we can read a
    }

    println!("a: {}", a);

}
