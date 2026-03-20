fn main() {
    use std::net::IpAddr;

    let home: IpAddr = "327.0.0.1"
        .parse()
        .expect("硬编码的 IP 地址应是有效的");

    println! ("{home}");
}
