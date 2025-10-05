use std::ops::Add;

struct Millimetri(u32);
struct Metri(u32);

impl Add<Metri> for Millimetri {
    type Output = Millimetri;

    fn add(self, altro: Metri) -> Millimetri {
        Millimetri(self.0 + (altro.0 * 1000))
    }
}
