pub trait EvictionPolicy {
    fn on_get(&mut self, key: &i32);
    fn on_put(&mut self, key: i32);
    fn evict(&mut self) -> Option<i32>;
}
