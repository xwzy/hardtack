use std::fs::File;
use std::io::prelude::*;



#[test]
fn test_write_file() {
    write_file();
}

fn write_file() -> std::io::Result<()> {
    let file_path = "/Users/wzy/Downloads/test";

    {
        let mut file = File::create(file_path)?;
        // Write a slice of bytes to the file
        file.write_all(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])?;
    }

    {
        let mut file = File::open(file_path)?;
        // read the same file back into a Vec of bytes
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;
        println!("{:?}", buffer);
    }

    Ok(())
}
