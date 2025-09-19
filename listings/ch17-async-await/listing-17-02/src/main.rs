extern crate trpl; // necessario per test mdbook

use trpl::Html;

fn main() {
    // TODO: lo aggiungeremo in seguito!
}

async fn titolo_pagina(url: &str) -> Option<String> {
    // ANCHOR: chaining
    let testo_risposta = trpl::get(url).await.text().await;
    // ANCHOR_END: chaining
    Html::parse(&testo_risposta)
        .select_first("title")
        .map(|titolo| titolo.inner_html())
}
