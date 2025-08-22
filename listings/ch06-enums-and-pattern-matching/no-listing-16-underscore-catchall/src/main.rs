fn main() {
    // ANCHOR: here
    let tiro_dadi = 9;
    match tiro_dadi {
        3 => metti_cappello_buffo(),
        7 => togli_cappello_buffo(),
        _ => tira_ancora(),
    }

    fn metti_cappello_buffo() {}
    fn togli_cappello_buffo() {}
    fn tira_ancora() {}
    // ANCHOR_END: here
}
