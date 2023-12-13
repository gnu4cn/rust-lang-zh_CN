fn main() {
    let s = String::from("hello");

    let len = s.len();

    let slice_1 = &s[3..len];
    let slice_2 = &s[3..];

    assert_eq! (slice_1, slice_2);
}
