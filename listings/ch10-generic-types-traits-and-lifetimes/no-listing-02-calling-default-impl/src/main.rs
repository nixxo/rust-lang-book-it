use aggregatore::{self, ArticoloNews, Sommario};

fn main() {
    // ANCHOR: here
    let articolo = ArticoloNews {
        titolo: String::from("I Penguins vincono la Stanley Cup!"),
        posizione: String::from("Pittsburgh, PA, USA"),
        autore: String::from("Iceburgh"),
        contenuto: String::from(
            "I Pittsburgh Penguins sono ancora una volta\
             la migliore squadra di hockey nella NHL.",
        ),
    };

    println!("Nuovo articolo disponibile! {}", articolo.riassunto());
    // ANCHOR_END: here
}
