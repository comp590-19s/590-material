struct Point {
    x: u32,
    y: u32
}

fn point_factory<'f>(x: u32, y: u32) -> &'f Point {
    let p = Point { x, y };
    return &p;
}

fn main() {
    println!("One Point struct, right away!");
    let a: &Point = point_factory(1, 0);
    println!("x: {}", a.x);
}
