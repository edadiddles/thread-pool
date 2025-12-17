mod threadpool;
mod task;
mod work_queue;

use crate::threadpool::ThreadPool;
use crate::task::Task;
use crate::work_queue::WorkQueue;

fn main() {
    let thread_pool = ThreadPool::new(255);
    println!("Num Threads: {}", thread_pool.num_threads);
    let _ = Task::new();
    let mut q = WorkQueue::new();
    q.push(1);
    let p = q.peek().unwrap();
    let v = q.pop().unwrap();
    println!("Peek: {}", p);
    println!("Pop: {}", v);

}
