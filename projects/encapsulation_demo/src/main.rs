use encapsulation_demo::AveragedCollection;

fn main() {
    let mut col = AveragedCollection::new();

    col.add(10);
    col.add(100);
    col.add(100);
    col.add(100);
    println! ("{}", col.average());

}
