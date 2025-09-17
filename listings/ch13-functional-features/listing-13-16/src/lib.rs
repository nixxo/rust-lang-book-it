#[derive(PartialEq, Debug)]
struct Scarpa {
    misura: u32,
    stile: String,
}

fn misura_scarpe(scarpe: Vec<Scarpa>, misura_scarpa: u32) -> Vec<Scarpa> {
    scarpe.into_iter().filter(|s| s.misura == misura_scarpa).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtra_per_misura() {
        let scarpe = vec![
            Scarpa {
                misura: 10,
                stile: String::from("sneaker"),
            },
            Scarpa {
                misura: 13,
                stile: String::from("sandalo"),
            },
            Scarpa {
                misura: 10,
                stile: String::from("scarpone"),
            },
        ];

        let della_mia_misura = misura_scarpe(scarpe, 10);

        assert_eq!(
            della_mia_misura,
            vec![
                Scarpa {
                    misura: 10,
                    stile: String::from("sneaker")
                },
                Scarpa {
                    misura: 10,
                    stile: String::from("scarpone")
                },
            ]
        );
    }
}
