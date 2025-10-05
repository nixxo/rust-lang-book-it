fn main() {
    // ANCHOR: here
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn prende_type_long(f: Thunk) {
        // --taglio--
    }

    fn ritorna_type_long() -> Thunk {
        // --taglio--
        // ANCHOR_END: here
        Box::new(|| ())
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
