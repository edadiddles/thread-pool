// queue should contain a handle to the method execution
pub struct WorkQueue {
    pub queue: [u32; 10],
    read_idx: usize,
    write_idx: usize,
    slot_cnt: usize,
}

impl WorkQueue {
    pub fn new() -> Self {
        Self {
            queue: [0; 10],
            read_idx: 0,
            write_idx: 0,
            slot_cnt: 0,
        }
    }

    pub fn push(&mut self, handle: u32) {
        if self.is_full() {
            println!("Queue Full");
            return;
        }

        self.queue[self.write_idx] = handle;
        self.increment_write_idx();
    }

    pub fn peek(&self) -> Result<u32, &'static str> {
        if self.is_empty() {
            println!("Queue Empty");
            return Err("Queue Empty");
        }

        Ok(self.queue[self.read_idx])
    }

    pub fn pop(&mut self) -> Result<u32, &'static str> {
        if self.is_empty() {
            println!("Queue Empty");
            return Err("Queue Empty");
        }

        let val = self.queue[self.read_idx];
        self.queue[self.read_idx] = 0;
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

        q.push(1);
        let v = q.peek().unwrap();
        println!("testing: {}", v);
        assert_eq!(1, v);
    }

    #[test]
    fn test_empty_pop() {
        let mut q = WorkQueue::new();

        let v = q.pop().unwrap_err();
        assert_eq!("Queue Empty", v);
    }

    #[test]
    fn test_full_push() {
        let mut q = WorkQueue::new();
        
        q.push(1);
        q.push(2);
        q.push(3);
        q.push(4);
        q.push(5);
        q.push(6);
        q.push(7);
        q.push(8);
        q.push(9);
        q.push(10); 
        q.push(11); // Queue Full

        let _ = q.pop().unwrap(); // 1
        let _ = q.pop().unwrap(); // 2
        let _ = q.pop().unwrap(); // 3
        let _ = q.pop().unwrap(); // 4
        let _ = q.pop().unwrap(); // 5
        let _ = q.pop().unwrap(); // 6
        let _ = q.pop().unwrap(); // 7
        let _ = q.pop().unwrap(); // 8
        let _ = q.pop().unwrap(); // 9
        let v = q.pop().unwrap(); // 10
        let e = q.pop().unwrap_err(); // Queue Empty

        assert_eq!(10, v);
        assert_eq!("Queue Empty", e);
    }

    #[test]
    fn test_moving_base_idx() {
        let mut q = WorkQueue::new();

        for _ in 1..=1000 {
            for m in 1..=9 {
                q.push(m);
            }

            for _ in 1..=9{
                let _ = q.pop().unwrap();
            }

            assert!(q.is_empty())
        }
    }

    #[test]
    fn test_acc_buffer() {
        let mut q = WorkQueue::new();

        for _ in 1..=9 {
            for m in 1..=2 {
                q.push(m);
            }

            let _ = q.pop().unwrap();
        }

        q.pop().unwrap();
        q.pop().unwrap();
        q.pop().unwrap();
        q.pop().unwrap();
        q.pop().unwrap();
        q.pop().unwrap();
        q.pop().unwrap();
        q.pop().unwrap();
        q.pop().unwrap();
        assert!(q.is_empty());
    }
}
