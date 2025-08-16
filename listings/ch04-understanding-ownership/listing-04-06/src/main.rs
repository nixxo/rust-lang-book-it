fn main() {
    let s = String::from("hello");

    cambia(&s);
}

fn cambia(una_stringa: &String) {
    una_stringa.push_str(", world");
}
