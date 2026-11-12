use std::{
    fs::File,
    io::{ErrorKind, Read},
};

fn main() {
    let path = "test.txt";
    let file = File::open(path);

    let mut file_desc = match file {
        Ok(file_descriptor) => file_descriptor,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(_) => panic!("Prolem creating the file {}", path),
            },
            _ => panic!("Problem opening the file {}!", path),
        },
    };

    let mut buf = String::new();
    let bytes_count = file_desc.read_to_string(&mut buf);
    match bytes_count {
        Ok(bytes_count) => {
            println!("Readed file with size: {} bytes", bytes_count);
            println!("{}", buf);
        }
        Err(_) => panic!("Problem reading data from the file!"),
    };
}
