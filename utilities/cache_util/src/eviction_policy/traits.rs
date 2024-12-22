pub trait EvictionPolicy {
    fn on_get(&mut self, key: &i32);
    fn on_put(&mut self, key: i32);
    fn evict(&mut self) -> Option<i32>;
}

pub trait EvictionAsStoragePolicy {
    fn on_get(&mut self, _key: &i32) {
        /* NO OP */
    }
    fn on_put(&mut self, key: i32);
    fn evict(&mut self) -> Option<i32>;

    fn put(&mut self, _key: i32, _value: i32) {
        /* NO OP */
    }
    fn get(&mut self, key: &i32) -> Option<i32>;
    fn remove(&mut self, _key: &i32) {
        /* NO OP */
    }
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
