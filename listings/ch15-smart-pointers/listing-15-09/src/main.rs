struct MioBox<T>(T);

impl<T> MioBox<T> {
    fn new(x: T) -> MioBox<T> {
        MioBox(x)
    }
}

// ANCHOR: here
fn main() {
    let x = 5;
    let y = MioBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
// ANCHOR_END: here
