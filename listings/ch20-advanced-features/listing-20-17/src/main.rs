// ANCHOR: here
trait Pilota {
    fn vola(&self);
}

trait Mago {
    fn vola(&self);
}

struct Umano;

impl Pilota for Umano {
    fn vola(&self) {
        println!("Qui parla il capitano.");
    }
}

impl Mago for Umano {
    fn vola(&self) {
        println!("Sali!");
    }
}

impl Umano {
    fn vola(&self) {
        println!("*sbatte furiosamente le braccia*");
    }
}
// ANCHOR_END: here

fn main() {}
