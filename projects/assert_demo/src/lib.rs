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
    fn add_two_and_two() {
        assert_eq! (4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq! (5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq! (102, add_two(100));
    }

    #[test]
    fn it_works() {
        assert_eq! (2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_ne! (100, nth_fibonacci(50));
    }
}
