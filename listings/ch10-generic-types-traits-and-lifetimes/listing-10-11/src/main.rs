struct Punto<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Punto<X1, Y1> {
    fn mixup<X2, Y2>(self, altro: Punto<X2, Y2>) -> Punto<X1, Y2> {
        Punto {
            x: self.x,
            y: altro.y,
        }
    }
}

fn main() {
    let p1 = Punto { x: 5, y: 10.4 };
    let p2 = Punto { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
