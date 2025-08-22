fn main() {
    // ANCHOR: here
    let tiro_dadi = 9;
    match tiro_dadi {
        3 => metti_cappello_buffo(),
        7 => togli_cappello_buffo(),
        altro => cambia_giocatore(altro),
    }

    fn metti_cappello_buffo() {}
    fn togli_cappello_buffo() {}
    fn cambia_giocatore(num_spazi: u8) {}
    // ANCHOR_END: here
}
