fn main() {
    let mut list = vec! [1, 2, 3];
    println! ("在定义闭包前：{:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println! ("在调用闭包后：{:?}", list);
}
