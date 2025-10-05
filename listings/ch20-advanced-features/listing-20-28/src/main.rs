fn più_uno(x: i32) -> i32 {
    x + 1
}

fn raddoppia(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let risposta = raddoppia(più_uno, 5);

    println!("La risposta è: {risposta}");
}
