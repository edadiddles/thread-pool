mod threadpool;
use crate::threadpool::ThreadPool;

fn main() {
    let thread_pool = ThreadPool::new(255);
    println!("{}", thread_pool.num_threads);
}
