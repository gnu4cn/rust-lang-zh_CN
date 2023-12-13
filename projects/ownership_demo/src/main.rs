fn main() {
    let s = String::from("hello");  // s 进到作用域

    takes_ownership(s);             // 变量 s 的值迁移到这个函数中......
                                    // ......进而在这里不再有效

    let x = 5;                      // x 进到作用域

    makes_copy(x);                  // x 将迁移到这个函数中，
                                    // 但由于 i32 实现了 `Copy` 特质，因此
                                    // 后面在使用变量 x 没有问题
    println! ("x = {x}");
}   // 到这里，x 超出作用域，接着是 s。但由于 s 的值已被迁移，因此
    // 不会有特别的事情发生。

fn takes_ownership(some_string: String) {   // some_string 进到作用域
    println! ("{}", some_string);
}   // 这里，some_string 超出作用域，而 `drop` 方法会被调用。退回的
    // 内存被释放。

fn makes_copy(some_integer: i32) {  // some_integer 进到作用域
    println! ("{}", some_integer);
}   // 这里，some_integer 超出作用域。没有特别事情发生。

