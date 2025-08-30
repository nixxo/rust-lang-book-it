pub struct Ipotesi {
    valore: i32,
}

impl Ipotesi {
    pub fn new(valore: i32) -> Ipotesi {
        // ANCHOR: here
        if valore < 1 {
            panic!(
                "L'ipotesi deve essere minore o uguale a 100, valore fornito {valore}."
            );
        } else if valore > 100 {
            panic!(
                "L'ipotesi deve essere maggiore di zero, valore fornito {valore}."
            );
        }
        // ANCHOR_END: here

        Ipotesi { valore }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "minore o uguale a 100")]
    fn maggiore_di_100() {
        Ipotesi::new(200);
    }
}
