use ciao_macro::CiaoMacro;
use ciao_macro_derive::CiaoMacro;

#[derive(CiaoMacro)]
struct Pancake;

fn main() {
    Pancake::ciao_macro();
}
