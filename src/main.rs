mod cache;
use std::time::Duration;
use cache::{CacheItem,InmemoryCache};
use std::thread::sleep;
use std::sync::{Arc,Mutex};
fn main() {
    let mut cache=Arc::new(Mutex::new(InmemoryCache::<String>::new()));   //turbofish syntax
   // let mut locked_cache=cache.lock().unwrap();   it will make the cache to be safe i.e lock till the end even when its sleep AND blocks the other threads completely.
//    so directly use the .lock() method while calling the fn , in this way when the scope ends it gets unlock and make avail for other threads
    
    let cache_clone=Arc::clone(&cache);
    std::thread::spawn(move || {
        loop {
            
            std::thread::sleep(Duration::from_secs(1));

            cache_clone.lock().unwrap().cleanup();

            println!("removed expired data");
        }

    });
    println!("inserting item with 2 sec ttl");
    cache.lock().unwrap().set(String::from("sathwik"), String::from("sathwik"), Duration::from_secs(2));
    match cache.lock().unwrap().get("sathwik"){
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
    match cache.lock().unwrap().get("sathwik"){
        Some(val)=>{
            println!("{}",val);
        }
        None=>{
            println!("expired");
        }
    }

    
}
