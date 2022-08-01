use super::value::Entry;
use super::ValueType;
use crate::tsdb::point::Point;
use crate::tsdb::TypeMismatchError;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{BuildHasherDefault, Hash, Hasher};
use tokio::sync::RwLock;
use twox_hash::XxHash64;

const STORE_LEN: usize = 16;

type XxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<XxHash64>>;
struct Partition(RwLock<XxHashMap<String, RwLock<Entry>>>);

impl Partition {
    pub fn new() -> Self {
        let inner = RwLock::new(XxHashMap::default());
        Self(inner)
    }
}

impl Default for Partition {
    fn default() -> Self {
        Partition::new()
    }
}

struct Cache {
    store: [Partition; STORE_LEN],
}

impl Cache {
    fn new() -> Self {
        let store: [Partition; 16] = Default::default();
        Self { store }
    }

    fn get_partition_mut(&mut self, series_key: &String) -> &mut Partition {
        let mut hasher = XxHash64::default();
        series_key.hash(&mut hasher);
        let hash = hasher.finish();
        let offset = hash % STORE_LEN as u64;
        &mut self.store[offset as usize]
    }
}
