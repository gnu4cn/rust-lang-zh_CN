pub fn add_two(a: u64) -> u64 {
    a + 3
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("二加二不等于四"))
        }
    }
}
