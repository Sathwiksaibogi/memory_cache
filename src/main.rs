mod cache;
use cache::{CacheItem,InmemoryCache};
fn main() {
    let mut cache=InmemoryCache::new();
    println!("Hello, world!");
}
