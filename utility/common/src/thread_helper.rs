//thread helper
use futures_cpupool::CpuPool;
use std::sync::mpsc::{channel, Receiver, Sender};
use threadpool::ThreadPool;

pub fn create_cpupool() -> CpuPool {
    let pool = CpuPool::new_num_cpus();
    
    pool
}

// ==================================
pub fn create_threadpool_jobs<T>(
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

// ==================================
pub fn create_async_threadpool_jobs<T> (
    jobs_num: usize,
    worker_num: usize,
    channel_capacity: usize, 
) -> Result<(ThreadPool, (tokio::sync::mpsc::Sender<T>, tokio::sync::mpsc::Receiver<T>)), Box<dyn std::error::Error>> {

    if jobs_num <= worker_num {
        return Err("too many job, it can cause deadlock".into());
    }

    let pool = ThreadPool::new(worker_num);
    let (sender, reciever) = tokio::sync::mpsc::channel::<T>(channel_capacity);

    Ok((pool, (sender, reciever)))
}

// ==================================

#[cfg(test)]
mod test_thread_helper {
    use std::thread;
    use tokio::runtime::Handle; 
    
    use super::*;

    const JOBS: usize = 4;
    const WORKERS: usize = 2;

    #[test]
    fn test_thread_pool() {
        let (pool, (sender, receiver)) = create_threadpool_jobs::<u8>(JOBS, WORKERS).unwrap();

        for _ in 0..JOBS {
            let sender_clone = sender.clone();
            pool.execute(move || {
                sender_clone
                    .send(1)
                    .expect("channel will be there waiting for the pool");
            });
        }

        assert_eq!(receiver.iter().take(JOBS).fold(0, |a, b| a + b), 4)
    }

    #[tokio::test]
    async fn test_async_thread() {

        let threads: Vec<_> = (0..3)
            .map(|thread_id|{
                let handle = Handle::current(); 
                thread::spawn(move || {
                    handle.spawn(async move {
                        thread_id
                    })
                })
            })
            .collect();

        let mut sum_of_ids = 0;
        for t in threads {
            let id  = t
            .join()
            .expect("error in join")
            .await
            .unwrap(); 

            sum_of_ids = sum_of_ids + id; 
        }
        assert_eq!(sum_of_ids, 3);
    }

}
