static mut COUNTER: u32 = 0;

/// 安全提示：同时从多个线程调用此函数属于未定义行为，
/// 因此咱们 *必须* 确保每次仅从单个线程调用他。

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // 安全提示：此函数仅在 `main` 中的单个线程中调用。
        add_to_count(3);
        println! ("COUNTER: {}", *(&raw const COUNTER));
    }
}
