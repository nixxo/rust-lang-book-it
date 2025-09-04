#[derive(Debug, PartialEq, Copy, Clone)]
enum ColoreMaglietta {
    Rosso,
    Blu,
}

struct Inventario {
    magliette: Vec<ColoreMaglietta>,
}

impl Inventario {
    fn giveaway(&self, user_preference: Option<ColoreMaglietta>) -> ColoreMaglietta {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ColoreMaglietta {
        let mut num_rosso = 0;
        let mut num_blu = 0;

        for color in &self.magliette {
            match color {
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
    let store = Inventario {
        magliette: vec![ColoreMaglietta::Blu, ColoreMaglietta::Rosso, ColoreMaglietta::Blu],
    };

    let user_pref1 = Some(ColoreMaglietta::Rosso);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "L'utente con preferenza {:?} riceve {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "L'utente con preferenza {:?} riceve {:?}",
        user_pref2, giveaway2
    );
}
