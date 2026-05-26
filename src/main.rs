mod cache;
use cache::{CacheItem,InmemoryCache};
fn main() {
    let mut cache=InmemoryCache::<String>::new();   //turbofish syntax
    println!("Hello, world!");
}
