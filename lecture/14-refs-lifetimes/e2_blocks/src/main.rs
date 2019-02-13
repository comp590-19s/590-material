fn main() {
    let mut x = 0;
    {
        x = 1;
        {
            let mut x = 2;
            x = 3;
            println!("{}", &x);
        }
        println!("{}", &x);
    }
    println!("{}", &x);
}
