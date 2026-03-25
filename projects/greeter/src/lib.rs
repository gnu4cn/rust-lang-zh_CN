pub fn greeting(name: &str) -> String {
    format! ("你好！")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Hector");
        assert! (
            result.contains("Hector"),
            "问候语未包含名字，值为 `{result}`",
        );
    }
}
