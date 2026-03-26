pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic! (
                "Guess 值必须大于或等于 1, 得到 {value}。"
            );
        } else if value > 100 {
            panic! (
                "Guess 值必须小于或等于 100, 得到 {value}。"
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "小于或等于 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
