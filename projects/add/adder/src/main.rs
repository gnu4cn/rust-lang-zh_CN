use add_one::add_one;
use add_two::add_two;

fn main() {
    let num = 10;
    println!("你好，世界！\n\t{num} 加 1 为 {}，{num} 加 2 为 {}！", 
        add_one(num), 
        add_two(num)
        );
}
