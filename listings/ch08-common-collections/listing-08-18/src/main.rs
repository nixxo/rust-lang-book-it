fn main() {
    // ANCHOR: here
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // nota che s1 è stato spostato qui e non può più essere utilizzato
    // ANCHOR_END: here
}
