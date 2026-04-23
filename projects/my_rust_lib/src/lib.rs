#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("刚刚从 C 调用了 Rust 函数！");
}

