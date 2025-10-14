struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p = Punto { x: 0, y: 7 };

    let Punto { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
