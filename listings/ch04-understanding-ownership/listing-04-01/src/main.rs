fn main() {
    // ANCHOR: here
    {                      // `s` non è valida qui, perché non ancora dichiarata
        let s = "hello";   // `s` è valida da questo punto in poi

        // fai cose con `s`
    }                      // questo scope è finito, e `s` non è più valida
    // ANCHOR_END: here
}
