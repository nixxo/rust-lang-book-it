// ANCHOR: here
use std::ops::Deref;

impl<T> Deref for MioBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// ANCHOR_END: here

struct MioBox<T>(T);

impl<T> MioBox<T> {
    fn new(x: T) -> MioBox<T> {
        MioBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MioBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
