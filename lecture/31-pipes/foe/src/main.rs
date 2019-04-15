use std::fs::File;
use std::io::prelude::*;

fn main()  -> std::io::Result<()> {

    let mut file = File::create("foe.txt")?;
    let mut i = 0u64;

    loop {
        i += 1;
        file.write_all(format!("file  : {}\n", i).as_bytes())?;
        println!("stdout: {}", i);
        eprintln!("stderr: {}", i);
    }

}
