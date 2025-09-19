extern crate trpl; // necessario per test mdbook

fn main() {
    // TODO: lo aggiungeremo in seguito!
}

// ANCHOR: all
use trpl::Html;

async fn titolo_pagina(url: &str) -> Option<String> {
    let risposta = trpl::get(url).await;
    let testo_risposta = risposta.text().await;
    Html::parse(&testo_risposta)
        .select_first("title")
        .map(|titolo| titolo.inner_html())
}
// ANCHOR_END: all
