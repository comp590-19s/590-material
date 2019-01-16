/**
 * This program iterates through each command-line argument
 * and prints each on a new line.
 */
fn main() {
    // Establish an iterator to the arguments passed to the program.
    let mut args = std::env::args();
    
    // Each argument is a string. We'll iterate through one-by-one.
    while let Some(arg) = args.next() {
        println!("{}", arg);
    }
}
