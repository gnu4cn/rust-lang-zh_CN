use std::io;
use rand::Rng;

fn main() {
    println! ("猜猜这个数!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println! ("秘密数字是：{secret_number}");

    println! ("请输入你的猜数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败！");

    println! ("你猜的是: {guess}");
}
