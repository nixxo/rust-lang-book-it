struct Punto<T> {
    x: T,
    y: T,
}

impl<T> Punto<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Punto { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
