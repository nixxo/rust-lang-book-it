fn main() {
    // ANCHOR: here
    unsafe fn pericolosa() {}

    unsafe {
        pericolosa();
    }
    // ANCHOR_END: here
}
