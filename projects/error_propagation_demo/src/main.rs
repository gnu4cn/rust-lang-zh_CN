use std::{fs, io};

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let result = match read_username_from_file() {
        Ok(res) => res,
        Err(e) => format! ("{e:?}")
    };

    println! ("{result}");
}


