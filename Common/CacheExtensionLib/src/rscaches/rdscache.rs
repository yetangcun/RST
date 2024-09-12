use redis::{Commands,Connection};
use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};
use deadpool_redis::{Config as RedisConfig, Pool, Runtime,Connection as pool_conn};


pub struct RdsPool {
    pool: Pool
}
impl RdsPool {
    pub fn new(url: &str, max_size:usize) -> Self {
        let cfg = RedisConfig::from_url(String::from(url));
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        RdsPool {
            pool
        }
    }

    pub async fn get_conn(&self) -> pool_conn {
        self.pool.get().await.unwrap()
    }
}

lazy_static! {
    static ref RDS_CACHE: Mutex<Connection> = Mutex::new(RdsCache::new());
}

pub struct RdsCache {}

impl RdsCache {
    pub fn new() -> Connection {
        let client = redis::Client::open("redis://192.168.30.166/").unwrap();
        let mut con = client.get_connection().unwrap();
        con
    }

    pub fn get_conn<'a>() -> MutexGuard<'a, Connection> {
        RDS_CACHE.lock().unwrap()
    }
}
