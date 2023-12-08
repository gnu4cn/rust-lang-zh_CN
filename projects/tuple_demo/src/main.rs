use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println! ("请输入一个数组索引。");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("读取行失败，failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("输入的所以并非一个数字");

    let element = a[index];

    println! ("位于索引 {index} 出的元素值为：{element}");
}
