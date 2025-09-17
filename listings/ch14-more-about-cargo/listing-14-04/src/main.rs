use arte::tipologia::ColorePrimario;
use arte::utilità::mix;

fn main() {
    let rosso = ColorePrimario::Rosso;
    let giallo = ColorePrimario::Giallo;
    mix(rosso, giallo);
}
