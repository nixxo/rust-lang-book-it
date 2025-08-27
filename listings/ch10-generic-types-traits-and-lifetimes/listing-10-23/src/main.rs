// ANCHOR: here
fn main() {
    let string1 = String::from("La stringa lunga è lunga");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("La stringa più lunga è {result}");
}
// ANCHOR_END: here

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
