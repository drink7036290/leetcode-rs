pub trait Cache {
    fn put(&mut self, key: i32, value: i32);
    fn get(&mut self, key: &i32) -> Option<i32>;
}
