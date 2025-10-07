fn stampa_coordinate(&(x, y): &(i32, i32)) {
    println!("Posizione corrente: ({x}, {y})");
}

fn main() {
    let punto = (3, 5);
    stampa_coordinate(&punto);
}
