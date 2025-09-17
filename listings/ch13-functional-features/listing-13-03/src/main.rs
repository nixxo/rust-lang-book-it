fn main() {
    // ANCHOR: here
    let esempio_chiusura = |x| x;

    let s = esempio_chiusura(String::from("ciao"));
    let n = esempio_chiusura(5);
    // ANCHOR_END: here
}
