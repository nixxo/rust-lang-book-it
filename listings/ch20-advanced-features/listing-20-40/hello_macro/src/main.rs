use hello_macro::HelloMacro;

struct Pancake;

impl HelloMacro for Pancake {
    fn hello_macro() {
        println!("Ciao, Macro! Il mio nome è Pancake!");
    }
}

fn main() {
    Pancake::hello_macro();
}
