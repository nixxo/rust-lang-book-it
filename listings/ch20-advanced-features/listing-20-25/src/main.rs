fn main() {
    // ANCHOR: here
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("ciao"));

    fn prende_type_long(f: Box<dyn Fn() + Send + 'static>) {
        // --taglio--
    }

    fn ritorna_type_long() -> Box<dyn Fn() + Send + 'static> {
        // --taglio--
        // ANCHOR_END: here
        Box::new(|| ())
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
