use super::*;

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq! (vec! ["safe, fast, productive."], search(query, contents));
}

#[test]
fn case_insensitive() {
    let query = "rUst";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq! (
        vec! ["Rust:", "Trust me."], 
        search_insensitive(query, contents)
    );
}
