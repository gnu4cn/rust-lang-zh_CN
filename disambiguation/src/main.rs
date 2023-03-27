trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("点点")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("狗崽")
    }
}

fn main() {
    println! ("小狗叫做 {}", <Dog as Animal>::baby_name());
}
