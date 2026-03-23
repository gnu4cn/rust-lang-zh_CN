#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("请叫我 Ishmael。数年前.....");
    let first_sentence = novel.split('。').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println! ("{i:?}");
}
