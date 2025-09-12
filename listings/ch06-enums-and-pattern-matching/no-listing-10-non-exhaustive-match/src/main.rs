fn main() {
    // ANCHOR: here
    fn più_uno(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
    // ANCHOR_END: here

    let cinque = Some(5);
    let sei = più_uno(cinque);
    let nulla = più_uno(None);
}
