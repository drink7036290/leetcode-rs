use super::HeapNodeTrait;
use std::cmp::Ordering;

pub struct ValueAwareHeapNode<H>
where
    H: HeapNodeTrait<Value = ()>,
{
    value: i32,
    node: H,
}

impl<H> PartialEq for ValueAwareHeapNode<H>
where
    H: HeapNodeTrait<Value = ()>,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.node == other.node
    }
}
impl<H> Eq for ValueAwareHeapNode<H> where H: HeapNodeTrait<Value = ()> {}

impl<H> PartialOrd for ValueAwareHeapNode<H>
where
    H: HeapNodeTrait<Value = ()>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<H> Ord for ValueAwareHeapNode<H>
where
    H: HeapNodeTrait<Value = ()>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.node.cmp(&other.node) // ignore value
    }
}

impl<H> HeapNodeTrait for ValueAwareHeapNode<H>
where
    H: HeapNodeTrait<Value = ()>,
{
    type Key = H::Key;
    type Value = i32;

    fn new(key: Self::Key, value: Self::Value) -> Self {
        Self {
            value,
            node: HeapNodeTrait::new(key, ()),
        }
    }

    fn key(&self) -> &Self::Key {
        self.node.key()
    }
    fn value(&self) -> &Self::Value {
        &self.value
    }
    fn set_value(&mut self, value: Self::Value) {
        self.value = value;
    }

    fn on_access(&mut self) {
        self.node.on_access();
    }
}
