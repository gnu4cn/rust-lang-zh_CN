fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // 带有显式的类型注解
    let heart_eyed_cat = '😻';
    let a: [i32; 5] = [-1, 0, 1, 2, 3];

    println! ("c 为 {c}, z 为 {z}, 爱心猫{heart_eyed_cat}");
    println! ("a 为 {:#?}", a);
}
