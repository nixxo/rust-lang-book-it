fn main() {
    // ANCHOR: here
    let mut s = String::from("ciao");

    {
        let r1 = &mut s;
    }   // qui `r1` esce dallo scope, quindi possiamo creare
        // un nuovo reference senza problemi

    let r2 = &mut s;
    // ANCHOR_END: here
}
