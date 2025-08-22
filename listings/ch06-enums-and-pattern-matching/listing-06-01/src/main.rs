fn main() {
    // ANCHOR: here
    enum VersioneIndirizzoIp {
        V4,
        V6,
    }

    struct IpAddr {
        tipo: VersioneIndirizzoIp,
        indirizzo: String,
    }

    let home = IpAddr {
        tipo: VersioneIndirizzoIp::V4,
        indirizzo: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        tipo: VersioneIndirizzoIp::V6,
        indirizzo: String::from("::1"),
    };
    // ANCHOR_END: here
}
