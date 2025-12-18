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
    #[should_panic]
    fn test_submit() {
        // verify closure runs by expecting panic and calling panic inside closure
        let t = Task::new(|| panic!("closure ran"));

        t.run();
    }
}
