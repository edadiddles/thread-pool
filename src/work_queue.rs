// queue should contain a handle to the method execution
pub struct WorkQueue {
    pub queue: [u32; 10],
    read_idx: usize,
    write_idx: usize,
}

impl WorkQueue {
    pub fn new() -> Self {
        Self {
            queue: [0; 10],
            read_idx: 0,
            write_idx: 0,
        }
    }

    pub fn push(&mut self, handle: u32) {
        if (self.write_idx == 9 && self.read_idx == 0) || (self.read_idx != 0 && self.write_idx == self.read_idx-1) {
            println!("Queue Full");
            return;
        }

        self.queue[self.write_idx] = handle;
        self.increment_write_idx();
    }

    pub fn peek(self: Self) -> Result<u32, &'static str> {
        if self.read_idx == self.write_idx {
            println!("Queue Empty");
            return Err("Queue Empty");
        }

        Ok(self.queue[self.read_idx])
    }

    pub fn pop(&mut self) -> Result<u32, &'static str> {
        if self.read_idx == self.write_idx {
            println!("Queue Empty");
            return Err("Queue Empty");
        }

        let val = self.queue[self.read_idx];
        self.queue[self.read_idx] = 0;
        self.increment_read_idx();
        Ok(val)
    }

    fn increment_write_idx(&mut self) {
        self.write_idx += 1;
        if self.write_idx == 10 {
            self.write_idx = 0;
        }
    }

    fn increment_read_idx(&mut self) {
        self.read_idx += 1;
        if self.read_idx == 10 {
            self.read_idx = 0;
        }
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
}
