// ANCHOR: here
fn ultimo_char_della_prima_riga(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
// ANCHOR_END: here

fn main() {
    assert_eq!(
        ultimo_char_della_prima_riga("Hello, world\nCome stai oggi?"),
        Some('d')
    );

    assert_eq!(ultimo_char_della_prima_riga(""), None);
    assert_eq!(ultimo_char_della_prima_riga("\nhi"), None);
}
