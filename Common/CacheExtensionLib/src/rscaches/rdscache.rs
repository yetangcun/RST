use redis::{Commands, Connection};
use deadpool_redis::{Config as RedisConfig, Pool, Runtime};


pub struct RdsPool {
    pool: Pool
}

impl RdsPool {
    pub fn new(url: &str, max_size:usize) -> Self {
        let cfg = RedisConfig::from_url(url).unwrap();
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        RdsPool {
            pool
        }
    }

    pub fn get_conn(&self) -> Connection {
        self.pool.get().unwrap()
    }
}