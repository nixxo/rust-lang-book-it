#[derive(PartialEq, Debug)]
struct Scarpa {
    size: u32,
    style: String,
}

fn misura_scarpe(scarpe: Vec<Scarpa>, misura_scarpa: u32) -> Vec<Scarpa> {
    scarpe.into_iter().filter(|s| s.size == misura_scarpa).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtra_per_misura() {
        let scarpe = vec![
            Scarpa {
                size: 10,
                style: String::from("sneaker"),
            },
            Scarpa {
                size: 13,
                style: String::from("sandalo"),
            },
            Scarpa {
                size: 10,
                style: String::from("scarpone"),
            },
        ];

        let della_mia_misura = misura_scarpe(scarpe, 10);

        assert_eq!(
            della_mia_misura,
            vec![
                Scarpa {
                    size: 10,
                    style: String::from("sneaker")
                },
                Scarpa {
                    size: 10,
                    style: String::from("scarpone")
                },
            ]
        );
    }
}
