use super::*;

#[test]
fn filter_by_size() {
    let shoes = vec! [
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq! (
        in_my_size,
        vec! [
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
        ]
    );
}
