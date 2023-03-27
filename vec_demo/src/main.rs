fn main() {
    let mut v = vec! [100, 32, 57];

    for i in &mut v {
        *i += 50;
        println! ("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec! [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    println! ("{:#?}", &row[0])

    // dbg! (v);
}
