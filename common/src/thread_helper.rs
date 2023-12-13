//thread helper
use std::sync::mpsc::{channel, Receiver, Sender};
use threadpool::ThreadPool;

pub fn create_jobs_jobs<T>(
    jobs_num: usize,
    worker_num: usize,
) -> Result<(ThreadPool, (Sender<T>, Receiver<T>)), Box<dyn std::error::Error>> {
    if jobs_num <= worker_num {
        return Err("too many job, it can cause deadlock".into());
    }

    let pool = ThreadPool::new(worker_num);
    let (sender, reciever) = channel::<T>();

    Ok((pool, (sender, reciever)))
}

#[cfg(test)]
mod test_thread_helper {    
    use super::*;

    #[test]
    fn test_thread_pool() {
        const JOBS: usize = 4; 
        const WORKERS: usize = 2;
        let (pool, (sender,receiver)) = create_jobs_jobs::<u8>(JOBS, WORKERS).unwrap();

        for _ in 0..JOBS {
            let sender_clone = sender.clone();
            pool.execute(move ||{
                sender_clone.send(1).expect("channel will be there waiting for the pool");
            });
        }

        assert_eq!(receiver.iter().take(JOBS).fold(0, |a,b| a+b), 4)
    }
}
