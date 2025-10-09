use ciao_macro::CiaoMacro;

struct Pancake;

impl CiaoMacro for Pancake {
    fn ciao_macro() {
        println!("Ciao, Macro! Il mio nome è Pancake!");
    }
}

fn main() {
    Pancake::ciao_macro();
}
