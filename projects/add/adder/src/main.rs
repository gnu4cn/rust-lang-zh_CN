fn main() {
    let num = 10;
    println!("
        你好，世界！
        {num} 加一为 {}!
        {num} 加二为 {}!
        ", add_one::add_one(num), add_two::add_two(num));
}
