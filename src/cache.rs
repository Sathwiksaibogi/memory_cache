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

    }

    pub fn get(&self,key:&str)->Option<&T>{

        None
    }
}