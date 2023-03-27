extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(input: f64) -> f64;
}


#[no_mangle]
pub extern "C" fn call_from_c() {
    println! ("刚从 C 调用了一个 Rust 函数！");
}

fn main() {
    unsafe {
        println! ("C 语言中 -3 的绝对值为：{}，3.0 的平方根为：{}", abs(-3), sqrt(3.0));
    }
}

