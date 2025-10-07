fn main() {
    let colore_preferito: Option<&str> = None;
    let e_martedi = false;
    let eta: Result<u8, _> = "34".parse();

    if let Some(color) = colore_preferito {
        println!("Usando il tuo colore preferito, {color}, come sfondo");
    } else if e_martedi {
        println!("Martedì è il giorno verde!");
    } else if let Ok(eta) = eta {
        if eta > 30 {
            println!("Usando il viola come colore di sfondo");
        } else {
            println!("Usando l'arancione come colore di sfondo");
        }
    } else {
        println!("Usando il blu come colore di sfondo");
    }
}
