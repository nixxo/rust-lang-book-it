extern crate trpl; // necessario per test mdbook

// ANCHOR: all
use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let titolo_fut_1 = titolo_pagina(&args[1]);
        let titolo_fut_2 = titolo_pagina(&args[2]);

        let (url, forse_titolo) =
            match trpl::select(titolo_fut_1, titolo_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} ritornato per primo");
        match forse_titolo {
            Some(titolo) => println!("Il suo titolo era: '{titolo}'"),
            None => println!("Non aveva titolo."),
        }
    })
}

async fn titolo_pagina(url: &str) -> (&str, Option<String>) {
    let testo_risposta = trpl::get(url).await.text().await;
    let titolo = Html::parse(&testo_risposta)
        .select_first("title")
        .map(|titolo| titolo.inner_html());
    (url, titolo)
}
// ANCHOR_END: all
