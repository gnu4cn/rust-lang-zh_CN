#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("闭包被调用");

    list.sort_by_key(|r| {
        sort_operations.push(&value);
        r.width
    });
    println!("
        {list:#?}
        {sort_operations:#?}
        ");
}
