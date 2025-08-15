fn main() {
    // ANCHOR: here
    {
        let s = String::from("hello"); // `s` è valida da questo punto in poi

        // fai cose con `s`
    }   // questo scope è finito, e `s` non è più valida
    // ANCHOR_END: here
}
