pub struct Ipotesi {
    valore: i32,
}

impl Ipotesi {
    pub fn new(valore: i32) -> Ipotesi {
        if valore < 1 || valore > 100 {
            panic!("L'ipotesi deve essere compresa tra 1 e 100, valore ottenuto: {valore}.");
        }

        Ipotesi { valore }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn maggiore_di_100() {
        Ipotesi::new(200);
    }
}
