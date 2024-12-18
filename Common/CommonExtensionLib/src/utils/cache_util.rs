use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;
use std::io::Error;

pub static STR_CACHES: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub static NUM_CACHES: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub fn set_str_cache(key: String, val: String) {
    STR_CACHES.lock().unwrap().insert(key, val);
}

pub fn get_str_cache(key: &str) -> Option<String> {
    let _cache = STR_CACHES.lock().unwrap();
    if let Some(val) = _cache.get(key) {
        return Some(val.clone());
    }
    return None;
    // STR_CACHES.lock().unwrap().get(&key).map(|s| s.to_string())
    // STR_CACHES.lock().unwrap().get(&key).map(|val| val.clone())
}

pub mod cache_moka_mod {
    use tokio;
    use std::future::Future;
    use moka::future::Cache;
    use std::time::Duration;
    use once_cell::sync::Lazy;
    use std::{sync::{Arc, Mutex}, collections::HashMap};

    // 添加一个包含值和过期时间的结构体
    #[derive(Clone)]
    struct CacheValue {
        value: String,
        expire_at: Option<u64>,
    }

    static CACHE_MOKA: Lazy<Cache<String, CacheValue>> = Lazy::new(|| {
        Cache::builder()
            .max_capacity(100000) // 设置最大容量为10万,如果不设置则是无限制
            //.time_to_live(Duration::from_secs(60)) // 设置过期时间为 60 秒
            .build()
    });
    
    pub async fn get(key: &str) -> Option<String> {
        let cache = CACHE_MOKA.clone();
        if let Some(cache_val) = cache.get(key).await {
            // 检查是否设置了过期时间
            if let Some(expire_at) = cache_val.expire_at {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                // 如果已过期，删除并返回 None
                if now > expire_at {
                    cache.invalidate(key).await;
                    return None;
                }
            }
            return Some(cache_val.value);
        }
        None
    }
    
    pub async fn set(key: String, val: String) {
        let cache_val = CacheValue {
            value: val,
            expire_at: None,
        };
        CACHE_MOKA.insert(key, cache_val).await;
    }

    pub async fn del(key: String){
        CACHE_MOKA.invalidate(&key).await;
    }

    pub async fn set_expire(key: String, val: String, expire: u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let cache_val = CacheValue {
            value: val,
            expire_at: Some(now + expire), // 计算过期时间点
        };
        CACHE_MOKA.insert(key, cache_val).await;
    }

    // 清空所有缓存
    pub fn clear() {
        CACHE_MOKA.invalidate_all();
    }

    // 获取缓存数
    pub fn get_counts() -> u64 {
        CACHE_MOKA.entry_count()
    }
}