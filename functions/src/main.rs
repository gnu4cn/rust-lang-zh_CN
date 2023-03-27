fn main() {
    let x = plus_one(-1);

    println! ("x 的值为：{}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
