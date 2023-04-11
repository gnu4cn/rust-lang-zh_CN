use rand::Rng;
use std::{cmp::Ordering, io, process};

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic! ("Guess 类型值必须在 1 与 100 之间，收到的是 {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    loop {
        println! ("\n---猜出这个数来！---");

        let secret_number: i32 = rand::thread_rng().gen_range(1..101);

        // println! ("随机生成的秘密数字为：{}", secret_number);

        loop {
            println! ("请输入你猜的数。（ ‘Q/quit’ 退出游戏）");

            let mut guess: String = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("读取行失败......");

            if guess.trim().eq("Q") || guess.trim().eq("quit") { process::exit(0); }

            // let guess: u32 = guess.trim().parse().expect("请输入一个数字！");
            let guess: i32 = match guess.trim().parse() {
               Ok(num) => num,
               Err(_) => { println! ("请输入一个数字！"); continue },
            };

            if guess < 1 || guess > 100 {
                println! ("秘密数字将在 1 和 100 之间");
                continue
            }

            println! ("你猜的数为：{}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println! ("太小了！"),
                Ordering::Greater => println! ("太大了！"),
                Ordering::Equal => {
                    println! ("你赢了！"); 
                    break
                },
            }
        }
    }
}
