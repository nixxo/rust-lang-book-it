enum Opzione<T> {
    Some(T),
    None,
}

use crate::Opzione::*;

// ANCHOR: here
impl<T> Opzione<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("chiamato `Opzione::unwrap()` su un valore `None`"),
        }
    }
}
// ANCHOR_END: here
