use std::fmt::Display;

struct Coppia<T> {
    x: T,
    y: T,
}

impl<T> Coppia<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Coppia<T> {
    fn mostra_comparazione(&self) {
        if self.x >= self.y {
            println!("Il membro più grande è x = {}", self.x);
        } else {
            println!("Il membro più grande è y = {}", self.y);
        }
    }
}
