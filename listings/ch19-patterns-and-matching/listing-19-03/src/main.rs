fn main() {
    let colore_preferito: Option<&str> = None;
    let è_martedi = false;
    let età: Result<u8, _> = "34".parse();

    if let Some(color) = colore_preferito {
        println!("Usando il tuo colore preferito, {color}, come sfondo");
    } else if è_martedi {
        println!("Martedì è il giorno verde!");
    } else if let Ok(età) = età {
        if età > 30 {
            println!("Usando il viola come colore di sfondo");
        } else {
            println!("Usando l'arancione come colore di sfondo");
        }
    } else {
        println!("Usando il blu come colore di sfondo");
    }
}
