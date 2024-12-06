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

    static CACHE_MOKA: Lazy<Cache<String, String>> = Lazy::new(|| {
        Cache::builder()
            //.max_capacity(100000) // 设置最大容量为10万
            //.time_to_live(Duration::from_secs(60)) // 设置 TTL 为 60 秒
            .build()
    });

    pub struct CacheMoka {
    }

    impl CacheMoka {

        pub async fn get(key: &str) -> Option<String> {
            let cache = CACHE_MOKA.clone();
            return cache.get(key).await;
        }

        pub async fn set(key: String, val: String){
            CACHE_MOKA.insert(key, val).await;
        }
        
        fn async_value<V>(value: V) -> impl Future<Output = V> 
        where V: Send + 'static,
        {
            Box::pin(async move { value })
        }

    }

}