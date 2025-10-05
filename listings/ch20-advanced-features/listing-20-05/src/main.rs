// ANCHOR: here
fn spezza_a_mut(valori: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = valori.len();

    assert!(mid <= len);

    (&mut valori[..mid], &mut valori[mid..])
}
// ANCHOR_END: here

fn main() {
    let mut vettore = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = spezza_a_mut(&mut vettore, 3);
}
