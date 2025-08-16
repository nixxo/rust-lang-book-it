fn main() {
    // ANCHOR: here
    let mut s = String::from("ciao");

    let r1 = &s; // nessun problema
    let r2 = &s; // nessun problema
    println!("{r1} and {r2}");
    // Le variabili `r1` e `r2` non verranno più usato dopo questo punto

    let r3 = &mut s; // nessun problema
    println!("{r3}");
    // ANCHOR_END: here
}
