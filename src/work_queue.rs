use crate::task::Task;

const QUEUE_SIZE: usize = 1000;

// queue should contain a handle to the method execution
pub struct WorkQueue {
    pub queue: [Option<Task>; QUEUE_SIZE],
    read_idx: usize,
    write_idx: usize,
    slot_cnt: usize,
}

impl WorkQueue {
    pub fn new() -> Self { 
        Self {
            queue: std::array::from_fn(|_| { None }),
            read_idx: 0,
            write_idx: 0,
            slot_cnt: 0,
        }
    }

    pub fn push(&mut self, handle: Task) {
        if self.is_full() {
            println!("Queue Full");
            return;
        }

        self.queue[self.write_idx] = Some(handle);
        self.increment_write_idx();
    }

    pub fn peek(&self) -> Result<&Task, &'static str> {
        if self.is_empty() {
            println!("Queue Empty");
            return Err("Queue Empty");
        }
        
        Ok(self.queue[self.read_idx].as_ref().unwrap())
    }

    pub fn pop(&mut self) -> Result<Task, &'static str> {
        if self.is_empty() {
            println!("Queue Empty");
            return Err("Queue Empty");
        }

        let val = self.queue[self.read_idx].take().unwrap();
        self.increment_read_idx();
        Ok(val)
    }

    fn is_full(&self) -> bool {
        self.slot_cnt == QUEUE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.slot_cnt == 0
    }

    fn increment_write_idx(&mut self) {
        self.write_idx += 1;
        if self.write_idx == QUEUE_SIZE {
            self.write_idx = 0;
        }
        self.slot_cnt += 1;
    }

    fn increment_read_idx(&mut self) {
        self.read_idx += 1;
        if self.read_idx == QUEUE_SIZE {
            self.read_idx = 0;
        }
        self.slot_cnt -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut q = WorkQueue::new();

        q.push(Task::new(|| println!("")));
        let v = q.pop().unwrap();
        v.run();
    }

    #[test]
    fn test_empty_pop() {
        let mut q = WorkQueue::new();
        assert!(q.pop().is_err());
    }

    #[test]
    fn test_full_push() {
        let mut q = WorkQueue::new();
       
        for _ in 1..=QUEUE_SIZE+1 {
            q.push(Task::new(|| {}));
        }

        for _ in 1..=QUEUE_SIZE {
            assert!(q.pop().is_ok());
        }

        // This is a pop on empty queue
        assert!(q.pop().is_err());
    }

    #[test]
    fn test_moving_base_idx() {
        let mut q = WorkQueue::new();

        // nested for loops ~queue_size^2, can take a little while to run on large queue_size
        for _ in 1..=QUEUE_SIZE*10 {
            for _ in 1..QUEUE_SIZE {
                q.push(Task::new(|| {}));
            }

            for _ in 1..QUEUE_SIZE{
                assert!(q.pop().is_ok());
            }

            assert!(q.is_empty())
        }
    }

    #[test]
    fn test_acc_buffer() {
        let mut q = WorkQueue::new();

        for _ in 1..QUEUE_SIZE {
            for _ in 1..=2 {
                q.push(Task::new(|| {}));
            }

            assert!(q.pop().is_ok());
        }

        for _ in 1..QUEUE_SIZE {
            assert!(q.pop().is_ok());
        }
        assert!(q.is_empty());
    }
}
