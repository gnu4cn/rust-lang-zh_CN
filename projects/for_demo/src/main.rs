fn main() {
    let v = vec! ['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println! ("{} 处于索引 {} 处", value, index);
    }
}
