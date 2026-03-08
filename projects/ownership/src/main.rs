fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 方法会追加一个字面值，到某个 String

    println! ("{}", s); // 这将打印出 `hello, world!`
}
