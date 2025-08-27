struct Punto<T, U> {
    x: T,
    y: U,
}

fn main() {
    let entrambi_interi = Punto { x: 5, y: 10 };
    let entrambi_float = Punto { x: 1.0, y: 4.0 };
    let intero_e_float = Punto { x: 5, y: 4.0 };
}
