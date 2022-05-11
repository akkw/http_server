use std::thread;
use std::time::Duration;
use http_pool::ThreadPool;
fn main() {
    let pool = ThreadPool::new(4);
    pool.executor(|| {
        println!("123")
    });
    pool.executor(|| {
        println!("123")
    });
    pool.executor(|| {
        println!("123")
    });
    pool.executor(|| {
        println!("123")
    });
    pool.executor(|| {
        println!("123")
    });
    pool.executor(|| {
        println!("123")
    });
    pool.executor(|| {
        println!("123")
    });


    thread::sleep(Duration::new(10,0))
}
