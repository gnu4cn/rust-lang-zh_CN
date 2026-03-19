use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt 应包含在此项目中");
}
