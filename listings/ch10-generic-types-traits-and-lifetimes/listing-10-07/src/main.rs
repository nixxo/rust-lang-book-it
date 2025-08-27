struct Punto<T> {
    x: T,
    y: T,
}

fn main() {
    let non_funzionante = Punto { x: 5, y: 4.0 };
}
