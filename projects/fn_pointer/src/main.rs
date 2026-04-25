fn main () {
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let mut list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    list_of_statuses.push(Status::Stop);

    println! ("{list_of_statuses:?}");
}
