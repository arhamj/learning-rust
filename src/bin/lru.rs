use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    let mut cache = LruCache::new(NonZeroUsize::new(2).unwrap());
    cache.put("key1", 3);
    cache.put("key2", 2);

    if cache.get("key2").is_some() {
        println!("key2: {:?}", cache.get("key2").unwrap());
    }
    cache.put("key3", 4);
    if cache.get("key3").is_some() {
        println!("key3: {:?}", cache.get("key3").unwrap());
    }
    if cache.get("key1").is_none() {
        println!("key1: Not found");
    }
}
