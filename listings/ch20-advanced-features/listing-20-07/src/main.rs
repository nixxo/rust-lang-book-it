fn main() {
    // ANCHOR: here
    use std::slice;

    let indirizzo = 0x01234usize;
    let r = indirizzo as *mut i32;

    let valori: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // ANCHOR_END: here
}
