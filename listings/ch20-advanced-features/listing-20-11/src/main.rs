static mut CONTATORE: u32 = 0;

/// SICUREZZA: Chiamarlo da più di un unico thread alla volta è un comportamento
/// non definito, *devi* quindi garantire cche verra chiamato da un singolo
/// thread alla volta
unsafe fn aggiungi_a_contatore(inc: u32) {
    unsafe {
        CONTATORE += inc;
    }
}

fn main() {
    unsafe {
        // SICUREZZA: È chiamato da un singolo threan in `main`.
        aggiungi_a_contatore(3);
        println!("CONTATORE: {}", *(&raw const CONTATORE));
    }
}
