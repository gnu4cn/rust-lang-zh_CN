fn main() {
    let s = String::from("The quick brown fox jumps over the lazy dog.");

    // `first_word` 工作于 String 的切片上，无论是部分还是整体
    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);
    println! ("{word}");
    // `first_word` 还工作于 String 的引用，这等同于 String 的整个切片
    let word = first_word(&s);
    println! ("{word}");

    let s_string_literal = "hello word";

    // `first_word` 工作于字符串字面值，不论部分还是整体
    let word = first_word(&s_string_literal[0..6]);
    let word = first_word(&s_string_literal[..]);
    println! ("{word}");

    // 由于字符串字面值已经 *是* 字符串切片，
    // 因此在无需切片语法下，这也会工作!
    let word = first_word(s_string_literal);
    println! ("{word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
