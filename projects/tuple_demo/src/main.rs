fn main() {
    let x = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println! ("
        x: {:?}
        x.0：{five_hundred}
        x.1：{six_point_four}
        x.2：{one}
    ", x)
}
