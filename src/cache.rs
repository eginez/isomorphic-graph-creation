use std::collections::HashMap;
use std::sync::{Arc,RwLock};

pub(crate) struct Cache<K, V> {
    cache: Arc<RwLock<HashMap<K, V>>>,
}

impl <K,V> Cache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{

    pub fn new() -> Self{
        Cache {
            cache: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn insert(&self, key: K, value: V) {
       let mut cache = self.cache.write().unwrap();
        cache.insert(key, value);
    }

    pub fn get(&self, key: K) -> Option<V> {
        let cache = self.cache.read().unwrap();
        cache.get(key).cloned()
    }


}