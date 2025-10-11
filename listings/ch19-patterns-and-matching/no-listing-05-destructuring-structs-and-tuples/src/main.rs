fn main() {
    struct Punto {
        x: i32,
        y: i32,
    }

    // ANCHOR: here
    let ((piedi, pollici), Punto { x, y }) = ((3, 10), Punto { x: 3, y: -10 });
    // ANCHOR_END: here
}
