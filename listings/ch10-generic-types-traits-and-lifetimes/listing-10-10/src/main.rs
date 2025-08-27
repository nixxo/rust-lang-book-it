struct Punto<T> {
    x: T,
    y: T,
}

impl<T> Punto<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// ANCHOR: here
impl Punto<f32> {
    fn distanza_da_origine(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// ANCHOR_END: here

fn main() {
    let p = Punto{ x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
