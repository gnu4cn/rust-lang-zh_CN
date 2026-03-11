#[derive(Debug)]
struct Color (i32, i32, i32);

#[derive(Debug)]
struct Point (i32, i32, i32);

fn main() {
    let black = Color (0, 0, 0);
    let white = Color (255, 255, 255);
    let origin = Point (0, 0, 0);

    println! ("
        black: {:?}
        white: {:?}
        origin: {:?}
    ", black, white, origin);

    let Point (x, y, z) = origin;
    println! ("origin: ({x}, {y}, {z})");
}
