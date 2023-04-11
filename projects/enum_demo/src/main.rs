fn main() {
    let config_max: Option<u8> = Some(3u8);
    
    if let Option::Some(max) =  (config_max) {
        println! ("极大值被设置为了 {}", max);
    }
}
