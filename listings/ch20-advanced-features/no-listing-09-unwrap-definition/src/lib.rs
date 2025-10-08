enum Option<T> {
    Some(T),
    None,
}

use crate::Option::*;

// ANCHOR: here
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("chiamato `Option::unwrap()` su un valore `None`"),
        }
    }
}
// ANCHOR_END: here
