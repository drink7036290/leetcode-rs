pub trait HeapNodeTrait: Ord {
    type Key;

    fn new(key: Self::Key) -> Self;
    fn key(&self) -> &Self::Key;

    fn on_access(&mut self);
}
