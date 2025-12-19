mod threadpool;
mod task;
mod work_queue;

use crate::threadpool::ThreadPool;
use crate::task::Task;
use crate::work_queue::WorkQueue;

fn main() {
    let thread_pool = ThreadPool::new(255);
    println!("Num Threads: {}", thread_pool.num_threads);
    let t = Task::new(|| println!("Hello task!"));
    let mut q = WorkQueue::new();
    q.push(t);
    let p = q.peek();
    println!("Peek: {}", p.is_ok());
    let v = q.pop();
    v.unwrap().run();

}
