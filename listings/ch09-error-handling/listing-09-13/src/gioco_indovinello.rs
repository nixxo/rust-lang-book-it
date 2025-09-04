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

    pub fn valore(&self) -> i32 {
        self.valore
    }
}
