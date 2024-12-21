pub trait CacheStorage {
    fn put(&mut self, key: i32, value: i32);
    fn get(&mut self, key: &i32) -> Option<i32>;
    fn remove(&mut self, key: &i32);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
