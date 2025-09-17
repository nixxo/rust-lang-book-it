// ANCHOR: here
//! # Arte
//!
//! Una libreria per modellare concetti artistici.

pub mod tipologia {
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
}

pub mod utilità {
    use crate::tipologia::*;

    /// Combina due colori primari in egual quantità
    /// per formare un colore secondario.
    pub fn mix(c1: ColorePrimario, c2: ColorePrimario) -> ColoreSecondario {
        ColoreSecondario::Arancio
    }
}
