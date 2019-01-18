fn main() {

    let mut a: Vec<u8> = vec![10, 20];
    println!("{:p}", &a);
    println!("{:p}", &a[0]);
    println!("{:p}", &a[1]);

    let p1: &mut Vec<u8> = &mut a;
    (*p1).push(30);

    let p2: &mut u8 = &mut a[2];
    *p2 = 40;
    println!("{:p}", p2);
    println!("{:?}", a);

}
