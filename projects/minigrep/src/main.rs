use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println! ("
        在文件 {file_path} 中
        检索 {query}
    ");

    let contents = fs::read_to_string(file_path)
        .expect("应该已经能够读取文件");

    println! ("带有文本：\n{contents}");
}
