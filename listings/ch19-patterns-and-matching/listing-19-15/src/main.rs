enum Messaggio {
    Esci,
    Muovi { x: i32, y: i32 },
    Scrivi(String),
    CambiaColore(i32, i32, i32),
}
// ANCHOR: here                                         
fn main() {
    let msg = Messaggio::CambiaColore(0, 160, 255);

    match msg {
        Messaggio::Esci => {
            println!("Il variante Esci non ha dati da destrutturare.");
        }
        Messaggio::Muovi { x, y } => {
            println!("Muovi in direzione x {x} e in direzione y {y}");
        }
        Messaggio::Scrivi(text) => {
            println!("Messaggio di testo: {text}");
        }
        Messaggio::CambiaColore(r, g, b) => {
            println!("Cambia colore in rosso {r}, verde {g}, e blu {b}");
        }
    }
}
