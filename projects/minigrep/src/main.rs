use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println! ("
        在文件 {file_path} 中
        检索 {query}
    ");
}
