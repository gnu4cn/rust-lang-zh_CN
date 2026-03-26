pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn nth_fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    } else {
        return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq! (4, result);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_ne! (100, nth_fibonacci(45));
    }
}
