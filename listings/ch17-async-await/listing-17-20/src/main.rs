extern crate trpl; // necessario per test mdbook

fn main() {
    trpl::run(async {
        // ANCHOR: here
        let a = async { 1u32 };
        let b = async { "Ciao!" };
        let c = async { true };

        let (risultato_a, risultato_b, risultato_c) = trpl::join!(a, b, c);
        println!("{risultato_a}, {risultato_b}, {risultato_c}");
        // ANCHOR_END: here
    });
}
