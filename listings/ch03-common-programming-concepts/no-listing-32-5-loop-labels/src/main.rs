fn main() {
    let mut conteggio = 0;
    'aumenta_conteggio: loop {
        println!("conteggio = {conteggio}");
        let mut rimanente = 10;

        loop {
            println!("rimanente = {rimanente}");
            if rimanente == 9 {
                break;
            }
            if conteggio == 2 {
                break 'aumenta_conteggio;
            }
            rimanente -= 1;
        }

        conteggio += 1;
    }
    println!("Fine conteggio = {conteggio}");
}
