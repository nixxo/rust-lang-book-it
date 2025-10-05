unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

fn main() {
    println!("Valore assoluto di -3 secondo C: {}", abs(-3));
}
