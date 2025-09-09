struct MioSmartPointer {
    data: String,
}

impl Drop for MioSmartPointer {
    fn drop(&mut self) {
        println!("Pulizia MioSmartPointer with data `{}`!", self.data);
    }
}

// ANCHOR: here
fn main() {
    let c = MioSmartPointer {
        data: String::from("alcuni dati"),
    };
    println!("MioSmartPointer creato.");
    c.drop();
    println!("MioSmartPointer pulito prima della fine di main.");
}
// ANCHOR_END: here
