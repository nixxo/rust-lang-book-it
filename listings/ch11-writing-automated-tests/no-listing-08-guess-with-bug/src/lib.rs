pub struct Ipotesi {
    valore: i32,
}

// ANCHOR: here
// --snip--
impl Ipotesi {
    pub fn new(valore: i32) -> Ipotesi {
        if valore < 1 {
            panic!("L'ipotesi deve essere compresa tra 1 e 100, valore ottenuto: {valore}.");
        }

        Ipotesi { valore }
    }
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn maggiore_di_100() {
        Ipotesi::new(200);
    }
}
