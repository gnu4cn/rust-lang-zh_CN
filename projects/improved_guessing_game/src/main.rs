use rand::Rng;
use std::{cmp::Ordering, io, process};

mod guessing_game;
use crate::guessing_game::Guess;

fn main() {
    loop {
        println! ("\n---猜出这个数来！---");

        let secret_number: i32 = rand::rng().random_range(1..101);

        // println! ("随机生成的秘密数字为：{}", secret_number);

        loop {
            println! ("请输入你猜的数。（ ‘Q/quit’ 退出游戏）");

            let mut guess: String = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("读取行失败/failed to read line");

            if guess.trim().eq("Q") || guess.trim().eq("quit") {
                process::exit(0);
            }

            // let guess: u32 = guess.trim().parse().expect("请输入一个数字！");
            let guess_number: i32 = match guess.trim().parse()
            {
                Ok(num) => num,
                Err(_) => {
                    println! ("请输入一个数字！");
                    continue
                },
            };

            let guess: Guess = Guess::new(guess_number);

            match guess.value().cmp(&secret_number) {
                Ordering::Less => println! ("太小！"),
                Ordering::Greater => println! ("太大！"),
                Ordering::Equal => {
                    println! ("你赢了！");
                    break
                },
            }
        }
    }
}

