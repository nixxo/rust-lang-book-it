fn main() {
    let numero = 6;

    if numero % 4 == 0 {
        println!("numero è divisibile per 4");
    } else if numero % 3 == 0 {
        println!("numero è divisibile per 3");
    } else if numero % 2 == 0 {
        println!("numero è divisibile per 2");
    } else {
        println!("numero non è divisibile per by 4, 3, o 2");
    }
}
