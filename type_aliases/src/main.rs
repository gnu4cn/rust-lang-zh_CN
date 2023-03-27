fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    assert_eq! (x, y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println! ("嗨"));

    fn takes_long_type(f: Thunk) {
        // --跳过代码--
    }

    fn returns_long_type() -> Thunk {
        // --跳过代码--
    }
}
