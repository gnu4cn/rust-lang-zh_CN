use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec! [34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println! ("最大数为 {result}");

    let char_list = vec! ['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println! ("最大字符为 {result}");

    let str_list = vec! ["one", "two", "three"];

    let result = largest(&str_list);
    println! ("最大字符串为 {result}");

}
