pub struct WorkQueue {
    pub queue: [u32; 10],
}

impl WorkQueue {
    pub fn new () -> Self {
        Self {
            queue: [0; 10],
        }
    }
}
