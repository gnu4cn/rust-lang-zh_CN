fn main() {

    let mut v = vec! [1, 2, 3, 4];

    let last = v.pop().unwrap();

    println!("{last}, {:?}", v);
}
