fn main() {
    let reference_a_nulla = pendente();
}

fn pendente() -> &String {
    let s = String::from("ciao");

    &s
}
