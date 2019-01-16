/**
 * This program emulates the standard `printenv` program.
 */
fn main() {
    // Establish an iterator to the environment variables
    let mut vars = std::env::vars();

    // Iterate through each key-value pair of environment variables.
    // Note that each item in the iterator is a tuple of (&str, &str).
    while let Some((name, value)) = vars.next() {
        println!("{}={}", name, value);
    }
}
