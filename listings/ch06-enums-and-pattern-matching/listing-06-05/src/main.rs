fn main() {
    // ANCHOR: here
    fn piu_uno(x: Option<i32>) -> Option<i32> {
        match x {
            // ANCHOR: first_arm
            None => None,
            // ANCHOR_END: first_arm
            // ANCHOR: second_arm
            Some(i) => Some(i + 1),
            // ANCHOR_END: second_arm
        }
    }

    let cinque = Some(5);
    let sei = piu_uno(cinque);
    let nulla = piu_uno(None);
    // ANCHOR_END: here
}
