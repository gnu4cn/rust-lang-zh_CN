unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}


#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("刚刚从 C 调用了 Rust 函数！");
}

fn main() {
    println! ("根据 C 语言，-3 的绝对值为：{}", abs(-3));
}
