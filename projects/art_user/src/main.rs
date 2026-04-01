use art::mix;
use art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let result = mix(red, yellow);

    println! ("{result:#?}");
}
