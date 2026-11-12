use std::{
    fs::File,
    io,
    io::{ErrorKind, Read},
};

fn read_file(path: &str) -> Result<(usize, String), io::Error> {
    let mut file_desc = File::open(path)?;
    let mut buf = String::new();
    let bytes = file_desc.read_to_string(&mut buf)?;

    Ok((bytes, buf))
}

fn main() {
    let (b_count, buf) = match read_file("test.txt") {
        Ok(res) => res,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Can't open file!"),
            ErrorKind::InvalidData => panic!("Unsiupported file encoding? only UTF-8 supported!"),
            _ => panic!("Unexpected error while reading the file"),
        },
    };
    println!("{}", b_count);
    println!("{}", buf);
}
