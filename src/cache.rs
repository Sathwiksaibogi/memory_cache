use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct CacheItem<T>{
    pub data: T,
    pub expiry: Instant,
}

pub struct InmemoryCache<T>{
    pub store:HashMap<String,CacheItem<T>>,
}

impl<T> InmemoryCache<T>{
    pub fn new()->Self{
        InmemoryCache{
            store:HashMap::new(),
        }
    }

    pub fn set(&mut self,key:String,val:T,ttl:Duration){

        let expiry=Instant::now() + ttl;
        let cacheitem=CacheItem{
            data:val,
            expiry,
        };
        self.store.insert(key,cacheitem );
    }

    pub fn get(&self,key:&str)->Option<&T>{
        match self.store.get(key){
            Some(item)=>{
                if Instant::now()>=item.expiry{
                    None
                } else{
                    Some(&item.data)
                }

            }

            None=>{
                None
            }
        }
    }
    pub fn cleanup(&mut self){
        let mut deadkeys=Vec::new();
        for (key,item) in self.store.iter(){
            if Instant::now() >= item.expiry {
                deadkeys.push(key.clone());
            }
        }
        for key in deadkeys{
            self.store.remove(&key);
        }
    }
}