fn main() {
    println!("{:?}", plans(21));
}

fn plans(x: u32) -> Result<String, &'static str> {
    let res = match bar(x) {
        Ok(b) => b,
        Err(s) => return Err(s)
    };
    Ok(format!("{}", res))
}

fn bar(age: u32) -> Result<bool, &'static str> {
    if age >= 21 {
        Ok(true)
    } else {
        Err("Yikes")
    }
}


