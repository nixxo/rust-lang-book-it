pub struct Post {
    stato: Option<Box<dyn Stato>>,
    contenuto: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            stato: Some(Box::new(Bozza {})),
            contenuto: String::new(),
        }
    }
}

trait Stato {}

struct Bozza {}

impl Stato for Bozza {}
