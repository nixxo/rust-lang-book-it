fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("La stringa più lunga è {result}");
}

// ANCHOR: here
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("una stringa molto lunga");
    result.as_str()
}
// ANCHOR_END: here
