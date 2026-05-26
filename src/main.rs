mod cache;
use std::time::Duration;
use cache::{CacheItem,InmemoryCache};
use std::thread::sleep;
fn main() {
    let mut cache=InmemoryCache::<String>::new();   //turbofish syntax
    println!("inserting item with 2 sec ttl");
    cache.set(String::from("sathwik"), String::from("sathwik"), Duration::from_secs(2));
    match cache.get("sathwik"){
        Some(val)=>{
            println!("{}",val);
        }
        None=>{
            println!("expired");
        }
    }

    println!("sleep for 3sec");
    sleep(Duration::from_secs(3));

    println!("fetching after sleep");
    match cache.get("sathwik"){
        Some(val)=>{
            println!("{}",val);
        }
        None=>{
            println!("expired");
        }
    }

    
}
