struct MioSmartPointer {
    data: String,
}

impl Drop for MioSmartPointer {
    fn drop(&mut self) {
        println!("Pulizia MioSmartPointer con dati `{}`!", self.data);
    }
}

// ANCHOR: here
fn main() {
    let c = MioSmartPointer {
        data: String::from("alcuni dati"),
    };
    println!("MioSmartPointer creato.");
    drop(c);
    println!("MioSmartPointer pulito prima della fine di main.");
}
// ANCHOR_END: here
