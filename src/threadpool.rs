pub struct ThreadPool {
    pub num_threads: u8,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let pool = Self {
            num_threads: num_threads,
        };

        pool
    }
}
