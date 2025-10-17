use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

// ANCHOR: here
pub struct ThreadPool {
    workers: Vec<Worker>,
    mittente: Option<mpsc::Sender<Job>>,
}
// --taglio--
// ANCHOR_END: here

type Job = Box<dyn FnOnce() + Send + 'static>;

// ANCHOR: here
impl ThreadPool {
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
        // --taglio--

        // ANCHOR_END: here
        assert!(dimensione > 0);

        let (mittente, ricevitore) = mpsc::channel();

        let ricevitore = Arc::new(Mutex::new(ricevitore));

        let mut workers = Vec::with_capacity(dimensione);

        for id in 0..dimensione {
            workers.push(Worker::new(id, Arc::clone(&ricevitore)));
        }

        // ANCHOR: here
        ThreadPool {
            workers,
            mittente: Some(mittente),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.mittente.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.mittente.take());

        for worker in self.workers.drain(..) {
            println!("Spegnimento worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
// ANCHOR_END: here

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

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
