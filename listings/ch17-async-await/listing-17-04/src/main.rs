extern crate trpl; // necessario per test mdbook

use trpl::Html;

// ANCHOR: run
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url = &args[1];
        match titolo_pagina(url).await {
            Some(titolo) => println!("Il titolo per {url} era {titolo}"),
            None => println!("{url} non aveva titolo"),
        }
    })
}
// ANCHOR_END: run

async fn titolo_pagina(url: &str) -> Option<String> {
    let testo_risposta = trpl::get(url).await.text().await;
    Html::parse(&testo_risposta)
        .select_first("title")
        .map(|titolo| titolo.inner_html())
}
