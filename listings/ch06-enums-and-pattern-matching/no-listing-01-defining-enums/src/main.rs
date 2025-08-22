// ANCHOR: def
enum VersioneIndirizzoIp {
    V4,
    V6,
}
// ANCHOR_END: def

fn main() {
    // ANCHOR: instance
    let quattro = VersioneIndirizzoIp::V4;
    let sei = VersioneIndirizzoIp::V6;
    // ANCHOR_END: instance

    // ANCHOR: fn_call
    instrada(VersioneIndirizzoIp::V4);
    instrada(VersioneIndirizzoIp::V6);
    // ANCHOR_END: fn_call
}

// ANCHOR: fn
fn instrada(verione_ip: VersioneIndirizzoIp) {}
// ANCHOR_END: fn
