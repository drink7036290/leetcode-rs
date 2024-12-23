pub trait HeapNodeTrait: Ord {
    type Key;
    type Value;

    fn new(key: Self::Key, value: Self::Value) -> Self;

    fn key(&self) -> &Self::Key;
    fn value(&self) -> &Self::Value;
    fn set_value(&mut self, value: Self::Value);

    fn on_access(&mut self);
}
