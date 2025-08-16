fn main() {
    let mut s = String::from("hello");

    cambia(&mut s);
}

fn cambia(una_stringa: &mut String) {
    una_stringa.push_str(", world");
}
