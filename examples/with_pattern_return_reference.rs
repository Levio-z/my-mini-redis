use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct SharedMap {
    inner: Arc<Mutex<SharedMapInner>>,
}

struct SharedMapInner {
    data: HashMap<i32, String>,
}

impl SharedMap {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SharedMapInner {
                data: HashMap::new(),
            })),
        }
    }

    pub fn insert(&self, key: i32, value: String) {
        let mut lock = self.inner.lock().unwrap();
        lock.data.insert(key, value);
    }

    pub fn with_value<F, T>(&self, key: i32, func: F) -> T
    where
        F: FnOnce(Option<&str>) -> T,
    {
        let lock = self.inner.lock().unwrap();
        func(lock.data.get(&key).map(|string| string.as_str()))
    }
}

fn main() {
    let shared = SharedMap::new();
    shared.insert(10, "foo".to_string());

    shared.with_value(10, |value| {
        println!("The value is {:?}.", value);
    });
}
