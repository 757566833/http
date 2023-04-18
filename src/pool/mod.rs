use std::thread::JoinHandle;

pub struct ThreadPool;
impl ThreadPool {
    pub fn  new (size :u64)->ThreadPool {
        ThreadPool
    }
    pub fn  execute <F, T>(self,f: F)
    where
    F: FnOnce() -> T,
    F: Send + 'static,
    {
        
    }
}