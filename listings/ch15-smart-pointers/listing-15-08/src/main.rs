// ANCHOR: here
struct MioBox<T>(T);

impl<T> MioBox<T> {
    fn new(x: T) -> MioBox<T> {
        MioBox(x)
    }
}
// ANCHOR_END: here

fn main() {}
