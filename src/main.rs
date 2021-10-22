// mod writer;


use std::fs::File;
use std::io::prelude::*;

fn write_hello() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello");
    Ok(())
}

fn main() {
    write_hello();
}
