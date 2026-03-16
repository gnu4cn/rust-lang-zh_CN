fn main() {
    let v = vec! [1, 2, 3, 4];

    let third: &i32 = &v[2];
    println! ("第三个元素为 {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println! ("第三个元素为 {third}"),
        None => println! ("没有第三个元素。"),
    }
}
