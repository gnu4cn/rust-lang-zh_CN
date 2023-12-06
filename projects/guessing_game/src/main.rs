use std::io;

fn main() {
    println! ("请猜这个数！");

    println! ("请输入你的猜数。");

    let mut guess = String::new();

    let bytes = io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败/failed to read line");

    println! ("你猜的是：{guess}，字节数：{bytes}");
}
