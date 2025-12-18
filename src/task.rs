pub struct Task {
    func_hndl: Box<dyn FnOnce()>,
}

impl Task {
    pub fn new(func_hndl: impl FnOnce() + 'static) -> Self {
        Self {
            func_hndl: Box::new(func_hndl),
        }
    }

    pub fn run(self: Self) {
        (self.func_hndl)();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit() {
        let t = Task::new(|| println!("Hello world"));

        t.run();
    }
}
