unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Valore assoluto di -3 secondo C: {}", abs(-3));
    }
}
