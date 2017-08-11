use std::sync::{Arc, Mutex};
use threadpool;

use chunk;

pub struct ThreadManager<M> {
    manager: Arc<Mutex<Box<M>>>,
    pool: threadpool::ThreadPool,
}

impl<M> ThreadManager<M>
    where M: chunk::Manager + Send + 'static
{
    pub fn new(threads: usize, manager: M) -> chunk::ChunkResult<ThreadManager<M>> {
        Ok(ThreadManager {
               manager: Arc::new(Mutex::new(Box::new(manager))),
               pool: threadpool::ThreadPool::new(threads),
           })
    }
}

impl<M> chunk::Manager for ThreadManager<M>
    where M: chunk::Manager + Send + 'static
{
    fn get_chunk<F>(&self, url: &str, start: u64, offset: u64, callback: F)
        where F: FnOnce(chunk::ChunkResult<Vec<u8>>) + Send + 'static
    {
        let manager = self.manager.clone();
        let url = url.to_owned();
        let start = start.clone();
        let offset = offset.clone();

        self.pool.execute(move || {
                              manager.lock().unwrap().get_chunk(&url, start, offset, callback);
                          });
    }
}