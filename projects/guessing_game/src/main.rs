use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println! ("请猜数！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println! ("秘密数字为：{secret_number}");

    println! ("请输入你的猜数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败/failed to read line");

    let guess: u32 = guess.trim().parse().expect("请输入一个数字！");

    println! ("你猜的是：{guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println! ("太小！"),
        Ordering::Greater => println! ("太大！"),
        Ordering::Equal => println! ("你赢了！"),
    }
}
