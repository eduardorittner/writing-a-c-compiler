use super::ValueStore;
use crate::token::{HANDLE_MAX, Handle};

impl<K: Handle<V> + Copy, V> ValueStore<K, V> for Vec<V> {
    fn new() -> Self {
        Vec::new()
    }

    fn with_capacity(cap: usize) -> Self {
        Vec::with_capacity(cap)
    }

    fn insert(&mut self, value: V) -> K {
        assert!((self.len() as u32) < HANDLE_MAX);
        self.push(value);
        ((self.len() - 1) as u32).into()
    }

    fn get(&mut self, key: &K) -> &V {
        self.get(key)
    }

    fn values(&self) -> impl Iterator {
        self.iter()
    }

    fn clear(&mut self) {
        self.clear()
    }
}
