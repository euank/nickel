use std::collections::{HashMap, HashSet};
use std::hash::{Hash, BuildHasher};

/// Implement HashMap functions on hashmaps that already have a hasher configured, like FxHashMap
// see https://internals.rust-lang.org/t/hashmap-set-new-with-capacity-and-buildhasher/15622/
pub trait HashMapExt {
    fn new() -> Self;
    fn with_capacity(x: usize) -> Self;
}

impl<K, V, S> HashMapExt for HashMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher + Default,
{
    fn new() -> Self {
        HashMap::with_hasher(S::default())
    }

    fn with_capacity(capacity: usize) -> Self {
        HashMap::with_capacity_and_hasher(capacity, S::default())
    }
}

impl<K, S> HashMapExt for HashSet<K, S>
where
    K: Hash + Eq,
    S: BuildHasher + Default,
{
    fn new() -> Self {
        HashSet::with_hasher(S::default())
    }

    fn with_capacity(capacity: usize) -> Self {
        HashSet::with_capacity_and_hasher(capacity, S::default())
    }
}
