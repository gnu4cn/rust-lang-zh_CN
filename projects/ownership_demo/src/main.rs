fn main() {
    let s = String::from("The quick brown fox jumps over the lazy dog.");

    // 函数 first_word 在 String 值的切片上有效，不管是部分还是全部的切片
    let word = first_word(&s[0..6]);
    println! ("{}", word);

    let word = first_word(&s[..]);
    println! ("{}", word);

    // 函数 first_word 还在 String 变量的引用上有效，而 String 变量的引用
    // 与 String 值的整个切片是等价的
    let word = first_word(&s);
    println! ("{}", word);

    let s_string_literal = "hello word";

    // 函数 first_word 在字符串字面值上有效，不论是部分还是整体
    let word = first_word(&s_string_literal[0..6]);
    println! ("{}", word);

    let word = first_word(&s_string_literal[..]);
    println! ("{}", word);

    // 由于字符串字面值已经 是 字符串切片，因此无需切片语法，这
    // 也是有效的!
    let word = first_word(s_string_literal);

    println! ("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
