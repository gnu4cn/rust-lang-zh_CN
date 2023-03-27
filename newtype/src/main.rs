use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! (f, "[{}]", self.0.join(", "))
    }
}
fn main() {
    let w = Wrapper(vec! [String::from("你好"), String::from("世界")]);
    println! ("w = {}", w);
}
