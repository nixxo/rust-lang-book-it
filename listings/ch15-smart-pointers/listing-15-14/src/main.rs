struct MioSmartPointer {
    data: String,
}

impl Drop for MioSmartPointer {
    fn drop(&mut self) {
        println!("Pulizia MioSmartPointer con dati `{}`!", self.data);
    }
}

fn main() {
    let c = MioSmartPointer {
        data: String::from("mia roba"),
    };
    let d = MioSmartPointer {
        data: String::from("altra roba"),
    };
    println!("MioSmartPointer creati.");
}
