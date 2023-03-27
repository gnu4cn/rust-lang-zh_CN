use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where 
    T: Display,
{
    println! ("通知！{}", ann);
    if x.len() > y.len() {
        x
    } else { 
        y 
    }
}

fn main() {
    let result = longest_with_an_announcement("abc", "测试", "计算结果已出来。");

    println! ("{}", result);
}
