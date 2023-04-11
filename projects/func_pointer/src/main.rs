fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}


fn main() {
    let answer = do_twice(add_one, 5);

    println! ("答案为：{}", answer);

    let list_of_numbers = vec! [1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

    println! ("结果为：{:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let mut list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    list_of_statuses.append(&mut vec! [Status::Stop]);
    println! ("list_of_statuses: {:?}", list_of_statuses);
}
