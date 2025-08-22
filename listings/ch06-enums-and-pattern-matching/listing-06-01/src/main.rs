fn main() {
    // ANCHOR: here
    enum VersioneIndirizzoIp {
        V4,
        V6,
    }

    struct IndirizzoIp {
        tipo: VersioneIndirizzoIp,
        indirizzo: String,
    }

    let home = IndirizzoIp {
        tipo: VersioneIndirizzoIp::V4,
        indirizzo: String::from("127.0.0.1"),
    };

    let loopback = IndirizzoIp {
        tipo: VersioneIndirizzoIp::V6,
        indirizzo: String::from("::1"),
    };
    // ANCHOR_END: here
}
