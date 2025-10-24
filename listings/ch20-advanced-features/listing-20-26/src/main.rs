fn main() {
    // ANCHOR: here
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("ciao"));

    fn prende_type_lungo(f: Thunk) {
        // --taglio--
    }

    fn ritorna_type_lungo() -> Thunk {
        // --taglio--
        // ANCHOR_END: here
        Box::new(|| ())
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
