fn foo(_: i32, y: i32) {
    println!("Questa funzione utilizza solo il parametro y: {y}");
}

fn main() {
    foo(3, 4);
}
