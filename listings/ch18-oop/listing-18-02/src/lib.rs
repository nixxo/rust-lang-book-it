pub struct CollezioneConMedia {
    lista: Vec<i32>,
    media: f64,
}

// ANCHOR: here
impl CollezioneConMedia {
    pub fn aggiungi(&mut self, valore: i32) {
        self.lista.push(valore);
        self.aggiorna_media();
    }

    pub fn rimuovi(&mut self) -> Option<i32> {
        let risultato = self.lista.pop();
        match risultato {
            Some(valore) => {
                self.aggiorna_media();
                Some(valore)
            }
            None => None,
        }
    }

    pub fn media(&self) -> f64 {
        self.media
    }

    fn aggiorna_media(&mut self) {
        let totale: i32 = self.lista.iter().sum();
        self.media = totale as f64 / self.lista.len() as f64;
    }
}
// ANCHOR_END: here
