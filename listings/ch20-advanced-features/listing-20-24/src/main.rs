use std::fmt;

struct Capsula(Vec<String>);

impl fmt::Display for Capsula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Capsula(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
