use std::{sync::mpsc, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    mittente: mpsc::Sender<Job>,
}

struct Job;

// ANCHOR: here
impl ThreadPool {
    // --taglio--
    // ANCHOR_END: here
    /// Crea un nuovo ThreadPool.
    ///
    /// La dimensione é il numero di thread nel gruppo.
    ///
    /// # Panics
    ///
    /// La funzione `new` genera panic se la dimensione é zero.
    // ANCHOR: here
    pub fn new(dimensione: usize) -> ThreadPool {
        assert!(dimensione > 0);

        let (mittente, ricevitore) = mpsc::channel();

        let mut workers = Vec::with_capacity(dimensione);

        for id in 0..dimensione {
            workers.push(Worker::new(id, ricevitore));
        }

        ThreadPool { workers, mittente }
    }
    // --taglio--
    // ANCHOR_END: here

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
    // ANCHOR: here
}

// --taglio--

// ANCHOR_END: here

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// ANCHOR: here
impl Worker {
    fn new(id: usize, ricevitore: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            ricevitore;
        });

        Worker { id, thread }
    }
}
// ANCHOR_END: here
