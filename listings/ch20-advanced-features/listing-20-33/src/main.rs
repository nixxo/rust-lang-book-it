fn main() {
    let handlers = vec![ritorna_chiusura(), ritorna_chiusura_inizializzata(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

fn ritorna_chiusura() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn ritorna_chiusura_inizializzata(init: i32) -> impl Fn(i32) -> i32 {
    move |x| x + init
}
