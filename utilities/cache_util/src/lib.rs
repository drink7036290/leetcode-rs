#![feature(map_many_mut)]

mod cache;
pub use cache::*;

mod generic_cache;
pub use generic_cache::*;

mod eviction_policy;
pub use eviction_policy::*;

mod cache_storage;
pub use cache_storage::*;

mod heap_node;
pub use heap_node::*;

mod mock;
pub use mock::*;
