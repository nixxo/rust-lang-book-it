enum Colore {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
    
enum Messaggio {
    Esci,
    Muovi { x: i32, y: i32 },
    Scrivi(String),
    CambiaColore(Colore),
}

fn main() {
    let msg = Messaggio::CambiaColore(Colore::Hsv(0, 160, 255));

    match msg {
        Messaggio::CambiaColore(Colore::Rgb(r, g, b)) => {
            println!("Cambia colore in rosso {r}, verde {g}, e blu {b}");
        }
        Messaggio::CambiaColore(Colore::Hsv(h, s, v)) => {
            println!("Cambia colore in tonalità {h}, saturazione {s}, valore {v}");
        }
        _ => (),
    }
}
