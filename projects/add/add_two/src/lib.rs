/// 将一个整数加 2。
///
/// # Examples
///
/// ```
/// let num = 15;
///
/// let answer = add_two::add_two(num);
/// assert_eq! (answer, 17);
/// ```
///
pub fn add_two(num: i32) -> i32 {
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(7);
        assert_eq!(result, 9);
    }
}
