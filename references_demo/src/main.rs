fn main() {
    let mut s = String::from("你好");

    let r1 = &s;
    {
        let r2 = &mut s;
        r2.push_str("，世界");
    }

    println! ("s = {}, r2 = {}", s, r1);
}
