// ANCHOR: here
//! # Arte
//!
//! Una libreria per modellare concetti artistici.

pub use self::tipologia::ColorePrimario;
pub use self::tipologia::ColoreSecondario;
pub use self::utilità::mix;

pub mod tipologia {
    // --taglio--
    // ANCHOR_END: here
    /// I colori primari secondo il modello RYB.
    pub enum ColorePrimario {
        Rosso,
        Giallo,
        Blu,
    }

    /// I colori secondari secondo il modello RYB.
    pub enum ColoreSecondario {
        Arancio,
        Verde,
        Viola,
    }
    // ANCHOR: here
}

pub mod utilità {
    // --taglio--
    // ANCHOR_END: here
    use crate::tipologia::*;

    /// Combina due colori primari in egual quantità
    /// per formare un colore secondario.
    pub fn mix(c1: ColorePrimario, c2: ColorePrimario) -> ColoreSecondario {
        ColoreSecondario::Arancio
    }
    // ANCHOR: here
}
// ANCHOR_END: here
