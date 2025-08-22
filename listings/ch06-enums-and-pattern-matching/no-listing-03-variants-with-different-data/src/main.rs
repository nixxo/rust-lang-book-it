fn main() {
    // ANCHOR: here
    enum IndirizzoIp {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IndirizzoIp::V4(127, 0, 0, 1);

    let loopback = IndirizzoIp::V6(String::from("::1"));
    // ANCHOR_END: here
}
