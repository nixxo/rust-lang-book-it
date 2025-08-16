fn main() {
    // ANCHOR: here
    let mut s = String::from("ciao");

    let r1 = &s;      // nessun problema
    let r2 = &s;      // nessun problema
    let r3 = &mut s;  // GROSSO PROBLEMA

    println!("{r1}, {r2}, e {r3}");
    // ANCHOR_END: here
}
