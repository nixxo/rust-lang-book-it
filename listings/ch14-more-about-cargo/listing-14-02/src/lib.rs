// ANCHOR: here
//! # Mio Crate
//!
//! `mio_crate` è una collezione di funzioni per compiere
//! certi calcoli più facilmente.

/// Aggiunge uno al numero dato.
// --taglio--
// ANCHOR_END: here
///
/// # Esempi
///
/// ```
/// let arg = 5;
/// let risposta = mio_crate::più_uno(arg);
///
/// assert_eq!(6, risposta);
/// ```
pub fn più_uno(x: i32) -> i32 {
    x + 1
}
