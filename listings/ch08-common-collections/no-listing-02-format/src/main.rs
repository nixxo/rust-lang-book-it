fn main() {
    // ANCHOR: here
    let s1 = String::from("uno");
    let s2 = String::from("due");
    let s3 = String::from("tre");

    let s = format!("{s1}-{s2}-{s3}");
    // ANCHOR_END: here
}
