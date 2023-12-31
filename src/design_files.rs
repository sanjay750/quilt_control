
use std::fs::File;
use std::io::{self, Read};




pub fn read_file(file_path: &str) -> io::Result<Vec<u8>> {

    let mut file = File::open(file_path)?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    Ok(contents)

}


