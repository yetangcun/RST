extern crate redis;
use redis::{Commands,Connection};
use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};
use deadpool_redis::{redis::AsyncCommands,Config as RedisConfig, Pool, Runtime,Connection as PoolConnection,PoolError};

// 复用连接对象--单例模式
lazy_static! {
    static ref RDS_CACHE: Mutex<Connection> = Mutex::new(
        RdsCache::bld_conn("redis://:xiaoxiao@192.168.30.166:6379/2")
        // RdsCache::new("redis://127.0.0.1:6379/2")
    );
    static ref RDS_POOL_CACHE: Mutex<Pool> = Mutex::new(
        RdsPool::get_pool("redis://:xiaoxiao@192.168.30.166:6379/2")
        // RdsPool::new("redis://127.0.0.1:6379/2")
    );
}

// 缓存连接池
pub struct RdsPool {}
impl RdsPool {
    pub fn tst_pool(&self) { // 方法，第一个参数总是self
        println!("RdsPool::new()");
    }

    pub fn get_pool(url: &str) -> Pool { // 关联函数
        let cfg = RedisConfig::from_url(String::from(url));
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        pool
    }

    pub async fn get_conn() -> PoolConnection { // 关联函数
        let pool = RDS_POOL_CACHE.lock().unwrap();
        pool.get().await.unwrap()
    }

    pub async fn set_str(ky:&str, vl:String) -> bool{ // 关联函数
        let mut conn:PoolConnection = Self::get_conn().await;
        let res = conn.set::<&str, String, bool>(ky, vl).await.unwrap();
        res
    }

    pub async fn get_str(key:&str) -> String { // 关联函数
        let mut conn:PoolConnection = Self::get_conn().await;
        conn.get(key).await.unwrap()
    }
}

// 引用redis库
pub struct RdsCache {}
impl RdsCache {
    pub fn tst_rds(&self) { // 方法，第一个参数总是self
        println!("RdsCache::new()");
    }
    
    pub fn bld_conn(url:&str) -> Connection {
        let client = redis::Client::open(url).unwrap();
        let mut con = client.get_connection().unwrap();
        con
    }

    pub fn get_conn<'a>() -> MutexGuard<'a, Connection> {
        RDS_CACHE.lock().unwrap()
    }

    pub fn set_str(ky:&str, vl:String) -> bool{
        let mut conn = Self::get_conn();
        let res = conn.set::<&str, String, bool>(ky, vl).unwrap();
        res
    }

    pub fn get_str(key:&str) -> String {
        let mut conn = Self::get_conn();
        conn.get(key).unwrap()
    }
}
