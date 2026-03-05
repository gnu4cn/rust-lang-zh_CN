use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println! ("猜猜这个数!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println! ("请输入你的猜数。");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取行失败！");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println! ("你猜的是: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println! ("太小！"),
            Ordering::Greater => println! ("太大！"),
            Ordering::Equal => {
                println! ("你赢了！");
                break
            }
        }
    }
}
