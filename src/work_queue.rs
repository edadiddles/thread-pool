use crate::task::Task;

// queue should contain a handle to the method execution
pub struct WorkQueue {
    pub queue: [Option<Task>; 10],
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
        self.slot_cnt == 10
    }

    fn is_empty(&self) -> bool {
        self.slot_cnt == 0
    }

    fn increment_write_idx(&mut self) {
        self.write_idx += 1;
        if self.write_idx == 10 {
            self.write_idx = 0;
        }
        self.slot_cnt += 1;
    }

    fn increment_read_idx(&mut self) {
        self.read_idx += 1;
        if self.read_idx == 10 {
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

        let v = q.pop();
        assert!(v.is_err());
    }

    #[test]
    fn test_full_push() {
        let mut q = WorkQueue::new();
        
        q.push(Task::new(|| println!("{}", 1)));
        q.push(Task::new(|| println!("{}", 2)));
        q.push(Task::new(|| println!("{}", 3)));
        q.push(Task::new(|| println!("{}", 4)));
        q.push(Task::new(|| println!("{}", 5)));
        q.push(Task::new(|| println!("{}", 6)));
        q.push(Task::new(|| println!("{}", 7)));
        q.push(Task::new(|| println!("{}", 8)));
        q.push(Task::new(|| println!("{}", 9)));
        q.push(Task::new(|| println!("{}", 10)));
        q.push(Task::new(|| println!("{}", 11))); // Queue Full

        let _ = q.pop(); // 1
        let _ = q.pop(); // 2
        let _ = q.pop(); // 3
        let _ = q.pop(); // 4
        let _ = q.pop(); // 5
        let _ = q.pop(); // 6
        let _ = q.pop(); // 7
        let _ = q.pop(); // 8
        let _ = q.pop(); // 9
        let v = q.pop(); // 10
        let e = q.pop(); // Queue Empty

        assert!(v.is_ok());
        assert!(e.is_err());
    }

    #[test]
    fn test_moving_base_idx() {
        let mut q = WorkQueue::new();

        for _ in 1..=1000 {
            for m in 1..=9 {
                q.push(Task::new(move || println!("{}", m)));
            }

            for _ in 1..=9{
                let _ = q.pop();
            }

            assert!(q.is_empty())
        }
    }

    #[test]
    fn test_acc_buffer() {
        let mut q = WorkQueue::new();

        for _ in 1..=9 {
            for m in 1..=2 {
                q.push(Task::new(move || println!("{}", m)));
            }

            let _ = q.pop();
        }

        let _ = q.pop();
        let _ = q.pop();
        let _ = q.pop();
        let _ = q.pop();
        let _ = q.pop();
        let _ = q.pop();
        let _ = q.pop();
        let _ = q.pop();
        let _ = q.pop();
        assert!(q.is_empty());
    }
}
