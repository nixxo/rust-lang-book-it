fn main() {
    // ANCHOR: here
    let tiro_dadi = 9;
    match tiro_dadi {
        3 => metti_cappello_buffo(),
        7 => togli_cappello_buffo(),
        _ => (),
    }

    fn metti_cappello_buffo() {}
    fn togli_cappello_buffo() {}
    // ANCHOR_END: here
}
