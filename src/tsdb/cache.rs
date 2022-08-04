use super::value::Entry;
use super::Result;
use crate::tsdb::point::Point;
use crate::tsdb::value::Row;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::sync::Arc;
use tokio::sync::RwLock;
use twox_hash::XxHash64;

const STORE_LEN: usize = 16;

type XxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<XxHash64>>;
struct Partition(RwLock<XxHashMap<String, Arc<RwLock<Entry>>>>);

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

#[derive(Clone)]
struct Cache {
    store: Arc<[Partition; STORE_LEN]>,
}

impl Cache {
    pub fn new() -> Self {
        let store: Arc<[Partition; 16]> = Default::default();
        Self { store }
    }

    fn get_partition_offset(&self, series_key: &String) -> usize {
        let mut hasher = XxHash64::default();
        series_key.hash(&mut hasher);
        let hash = hasher.finish();
        hash as usize % self.store.len()
    }

    pub async fn write_points(&mut self, points: &Vec<Point>) -> Result<()> {
        let mut series: XxHashMap<String, Entry> = XxHashMap::default();
        for point in points {
            let unix_nano = point.unix_nano();
            let fields = point.fields();
            for field in fields {
                let row = Row::new(unix_nano, (&field.value).into());
                series
                    .entry(field.series_id)
                    .or_insert(Entry::new(field.value.value_type()))
                    .push(row)?;
            }
        }

        for (id, mut new) in series {
            let offset = self.get_partition_offset(&id);
            let guard = self.store[offset].0.read().await;
            let entry = guard.get(&id);
            if let Some(entry) = entry {
                let entry = entry.clone();
                drop(guard);
                entry.write().await.append(&mut new)?;
            } else {
                drop(guard);
                let mut guard = self.store[offset].0.write().await;
                guard.entry(id).or_insert(Arc::new(RwLock::new(new)));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tsdb::cache::Cache;
    use crate::tsdb::point::{FieldValue, Point};
    use std::collections::BTreeMap;
    use std::time;

    #[tokio::test]
    async fn test_cache() {
        let cache = Cache::new();
        let time = time::SystemTime::now();
        let tags = BTreeMap::from([
            ("host".to_string(), "A".to_string()),
            ("cpu".to_string(), "0".to_string()),
        ]);
        let p = Point {
            measurement: "cpu".to_string(),
            tags,
            time,
            field: BTreeMap::from([("value".to_string(), FieldValue::I64(10))]),
        };

        let mut handlers = vec![];
        for _ in 0..10 {
            let p = vec![p.clone()];
            let mut cache = cache.clone();
            handlers.push(tokio::spawn(async move {
                cache.write_points(&p).await.unwrap();
            }));
        }
        futures::future::join_all(handlers).await;

        let series = &p.fields()[0].series_id;
        let offset = cache.get_partition_offset(series);
        let partition = cache.store[offset].0.read().await;
        let entry = partition.get(series).unwrap();
        let len = entry.read().await.len();
        assert_eq!(len, 10);
    }
}
