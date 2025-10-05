use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}

impl Add for Punto {
    type Output = Punto;

    fn add(self, altro: Punto) -> Punto {
        Punto {
            x: self.x + altro.x,
            y: self.y + altro.y,
        }
    }
}

fn main() {
    assert_eq!(
        Punto { x: 1, y: 0 } + Punto { x: 2, y: 3 },
        Punto { x: 3, y: 3 }
    );
}
