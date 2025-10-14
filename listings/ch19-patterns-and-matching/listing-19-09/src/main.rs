fn main() {
    let un_valore_option: Option<i32> = None;
    // ANCHOR: here
    let Some(x) = un_valore_option else {
        return;
    };
    // ANCHOR_END: here
}
