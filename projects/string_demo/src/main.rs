fn main() {
    let data = "初始内容";

    let s = data.to_string();

    // 该方法直接在字面值之上也工作
    let s = "初始内容".to_string();

    println! ("{s}");
}
