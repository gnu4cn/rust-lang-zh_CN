pub fn add_two(a: u64) -> u64 {
    internal_add(a, 2)
}

fn internal_add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_add(2, 2);
        assert_eq! (4, result);
    }
}
