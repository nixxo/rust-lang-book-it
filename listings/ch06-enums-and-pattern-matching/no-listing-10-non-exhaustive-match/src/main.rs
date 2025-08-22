fn main() {
    // ANCHOR: here
    fn piu_uno(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
    // ANCHOR_END: here

    let cinque = Some(5);
    let sei = piu_uno(cinque);
    let nulla = piu_uno(None);
}
