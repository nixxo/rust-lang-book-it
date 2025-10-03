use gui::Schermo;

fn main() {
    let schermo = Schermo {
        componenti: vec![Box::new(String::from("Ciao"))],
    };

    schermo.esegui();
}
