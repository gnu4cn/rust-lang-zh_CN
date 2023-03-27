use std::io;
use std::process;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println! ("请输入一个数组索引。");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("读取行失败");

    let index: usize = match index.trim()
        .parse() {
            Ok(num) => num,
            Err(_) => { 
                println! ("输入的索引并非数字");
                process::exit(0);
            }
        };

    let element = a[index];

    println! (
        "位于索引 {} 处的元素值为：{}",
        index, element);
}
