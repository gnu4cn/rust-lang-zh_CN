fn main() {
    unsafe fn dangerous() {
        println! ("这是一个不安全函数。");
    }

    dangerous();
}
