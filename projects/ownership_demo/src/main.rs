fn main() {
    let s1 = String::from("hello");

    let length = calculate_length(&s1);

    println! ("字符串 '{}' 的长度为：{}", s1, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

