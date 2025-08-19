// ANCHOR: all
fn main() {
    let larghezza1 = 30;
    let altezza1 = 50;

    println!(
        "L'area del rettangolo è di {} pixel quadrati.",
        area(larghezza1, altezza1)
    );
}

// ANCHOR: here
fn area(larghezza: u32, altezza: u32) -> u32 {
    // ANCHOR_END: here
    larghezza * altezza
}
// ANCHOR_END: all
