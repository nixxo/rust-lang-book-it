fn main() {
    // ANCHOR: here
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Indirizzo IP definito dovrebbe essere valido");
    // ANCHOR_END: here
}
