pub struct ThreadPool;

// ANCHOR: here
impl ThreadPool {
    // --taglio--
    // ANCHOR_END: here
    pub fn new(dimensione: usize) -> ThreadPool {
        ThreadPool
    }

    // ANCHOR: here
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
// ANCHOR_END: here
