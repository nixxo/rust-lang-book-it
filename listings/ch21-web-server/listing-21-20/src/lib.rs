use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    mittente: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Crea un nuovo ThreadPool.
    ///
    /// La dimensione é il numero di thread nel gruppo.
    ///
    /// # Panics
    ///
    /// La funzione `new` genera panic se la dimensione é zero.
    pub fn new(dimensione: usize) -> ThreadPool {
        assert!(dimensione > 0);

        let (mittente, ricevitore) = mpsc::channel();

        let ricevitore = Arc::new(Mutex::new(ricevitore));

        let mut workers = Vec::with_capacity(dimensione);

        for id in 0..dimensione {
            workers.push(Worker::new(id, Arc::clone(&ricevitore)));
        }

        ThreadPool { workers, mittente }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.mittente.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// ANCHOR: here
// --taglio--

impl Worker {
    fn new(id: usize, ricevitore: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = ricevitore.lock().unwrap().recv().unwrap();

                println!("Worker {id} ha un lavoro; in esecuzione.");

                job();
            }
        });

        Worker { id, thread }
    }
}
// ANCHOR_END: here
