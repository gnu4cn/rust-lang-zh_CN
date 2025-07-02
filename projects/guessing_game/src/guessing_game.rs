pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("猜数值必须在 1 与 100 之间，得到了 {value}。");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
