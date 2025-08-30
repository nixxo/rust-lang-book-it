pub struct Ipotesi {
    valore: i32,
}

// ANCHOR: here
// --snip--

impl Ipotesi {
    pub fn new(valore: i32) -> Ipotesi {
        if valore < 1 {
            panic!(
                "L'ipotesi deve essere maggiore di zero, valore fornito {valore}."
            );
        } else if valore > 100 {
            panic!(
                "L'ipotesi deve essere minore o uguale a 100, valore fornito {valore}."
            );
        }

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
// ANCHOR_END: here
