#[derive(Debug, PartialEq, Copy, Clone)]
enum ColoreMaglietta {
    Rosso,
    Blu,
}

struct Inventario {
    magliette: Vec<ColoreMaglietta>,
}

impl Inventario {
    fn regalo(&self, preferenze_utente: Option<ColoreMaglietta>) -> ColoreMaglietta {
        preferenze_utente.unwrap_or_else(|| self.maggior_stock())
    }

    fn maggior_stock(&self) -> ColoreMaglietta {
        let mut num_rosso = 0;
        let mut num_blu = 0;

        for colore in &self.magliette {
            match colore {
                ColoreMaglietta::Rosso => num_rosso += 1,
                ColoreMaglietta::Blu => num_blu += 1,
            }
        }
        if num_rosso > num_blu {
            ColoreMaglietta::Rosso
        } else {
            ColoreMaglietta::Blu
        }
    }
}

fn main() {
    let negozio = Inventario {
        magliette: vec![ColoreMaglietta::Blu, ColoreMaglietta::Rosso, ColoreMaglietta::Blu],
    };

    let pref_utente1 = Some(ColoreMaglietta::Rosso);
    let regalo1 = negozio.regalo(pref_utente1);
    println!(
        "L'utente con preferenza {:?} riceve {:?}",
        pref_utente1, regalo1
    );

    let pref_utente2 = None;
    let regalo2 = negozio.regalo(pref_utente2);
    println!(
        "L'utente con preferenza {:?} riceve {:?}",
        pref_utente2, regalo2
    );
}
