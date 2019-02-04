fn main() {
    let a: char = 'a';
    let a_ref: &char = &a;

    let b: Box<char> = Box::new('b');
    let b_ref: &Box<char> = &b;
    let b_content_ref: &char = &(*b);

    println!("a:             {}", a);
    println!("a_ref:         {:p}", a_ref);
    println!("b:             {}", b);
    println!("b_ref:         {:p}", b_ref);
    println!("b_content_ref: {:p}", b_content_ref);
}
