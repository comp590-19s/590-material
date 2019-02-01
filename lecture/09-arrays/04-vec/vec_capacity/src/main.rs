fn main() {
    let mut a_vec: Vec<u64> = Vec::with_capacity(1);

    // Push additional values onto the Vec and print
    // the address of its first element...
    for i in 0..9 {
        a_vec.push(i);
        println!("{} {:p}", i, &a_vec[0]);
    }
}
