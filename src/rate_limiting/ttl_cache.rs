use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use time::OffsetDateTime;

#[inline]
fn now() -> i128 {
    OffsetDateTime::now_utc().unix_timestamp_nanos() / 1000
}

/// A time-to-live entry that should remain available until the provided `ttl` value.
///
/// These are used to store order ids, `id`, and the creation time, `data`, of orders, but was left
/// generic for potential later use.
#[derive(Debug, Clone, Copy)]
pub struct TtlEntry<K, T>
where
    K: Ord + Clone,
    T: Clone,
{
    pub id: K,
    ttl: i128,
    pub data: T,
}

impl<K, T> TtlEntry<K, T>
where
    K: Ord + Clone,
    T: Clone,
{
    pub fn new(id: K, ttl_us: i128, data: T) -> TtlEntry<K, T> {
        TtlEntry {
            id,
            ttl: now() + ttl_us,
            data,
        }
    }
}

impl<K, T> Eq for TtlEntry<K, T>
where
    K: Ord + Clone,
    T: Clone,
{
}

impl<K, T> PartialEq<Self> for TtlEntry<K, T>
where
    K: Ord + Clone,
    T: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ttl == other.ttl
    }
}

impl<K, T> PartialOrd<Self> for TtlEntry<K, T>
where
    K: Ord + Clone,
    T: Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.ttl.cmp(&other.ttl))
    }
}

impl<K, T> Ord for TtlEntry<K, T>
where
    K: Ord + Clone,
    T: Clone,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.ttl.cmp(&other.ttl)
    }
}

/// A time-to-live cache that removes values when they expire. This is used to store and look up
/// orders, to determine how old they are for rate limiting penalties when editing or cancelling.
pub struct TtlCache<K, T>
where
    K: Ord + Clone,
    T: Clone,
{
    ids: BTreeMap<K, TtlEntry<K, T>>,
    ttls: BTreeSet<TtlEntry<K, T>>,
}

impl<K, T> Default for TtlCache<K, T>
where
    K: Ord + Clone,
    T: Clone,
{
    fn default() -> Self {
        TtlCache::new()
    }
}

impl<K, T> TtlCache<K, T>
where
    K: Ord + Clone,
    T: Clone,
{
    /// Create a new, empty cache.
    pub fn new() -> TtlCache<K, T> {
        TtlCache {
            ids: Default::default(),
            ttls: Default::default(),
        }
    }

    /// Insert the provided [TtlEntry] by it's id for future lookup. Entries beyond their ttl are
    /// removed automatically any time the `remove`, `get`, or `contains` methods are called.
    pub fn insert(&mut self, ttl_entry: TtlEntry<K, T>) -> Option<TtlEntry<K, T>> {
        self.ttls.insert(ttl_entry.clone());
        self.ids.insert(ttl_entry.id.clone(), ttl_entry)
    }

    /// Removes an entry manually, returning if the entry was removed.
    ///
    /// The cache is cleaned of any expired values after checking if this value was removed.
    ///
    /// This follows the same semantics as [BTreeSet]'s `remove` method.
    pub fn remove(&mut self, ttl_entry: &TtlEntry<K, T>) -> bool {
        self.ids.remove(&ttl_entry.id);
        let removed = self.ttls.remove(ttl_entry);
        self.remove_expired_values();

        removed
    }

    /// Returns if the provided key is in the cache, after removing any expired values.
    pub fn contains(&mut self, id: &K) -> bool {
        self.remove_expired_values();
        self.ids.contains_key(id)
    }

    /// Gets a [TtlEntry] by id after removing any expired values.
    pub fn get(&mut self, id: &K) -> Option<&TtlEntry<K, T>> {
        self.remove_expired_values();
        self.ids.get(id)
    }

    fn remove_expired_values(&mut self) {
        let now = now();
        let mut to_remove = Vec::new();

        for entry in &self.ttls {
            if entry.ttl < now {
                to_remove.push(entry.clone());
            }
        }

        for entry in to_remove {
            self.ids.remove(&entry.id);
            self.ttls.remove(&entry);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rate_limiting::ttl_cache::{TtlCache, TtlEntry};
    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::thread::sleep;
    use std::time::Duration as StdDuration;
    use time::Duration;

    #[test]
    fn test_ttl_entry_eq_partial_cmp() {
        let entry_0 = TtlEntry {
            id: "0x1",
            ttl: 0,
            data: 0,
        };
        let entry_1 = TtlEntry {
            id: "0x1",
            ttl: 1,
            data: 0,
        };
        let entry_2 = TtlEntry {
            id: "0x1",
            ttl: 1,
            data: 0,
        };

        assert_ne!(entry_0, entry_1);
        assert_ne!(entry_0, entry_2);
        assert_eq!(entry_1, entry_2);

        assert_eq!(Less, entry_0.partial_cmp(&entry_1).unwrap());
        assert_eq!(Less, entry_0.partial_cmp(&entry_2).unwrap());
        assert_eq!(Equal, entry_1.partial_cmp(&entry_2).unwrap());
    }

    #[test]
    fn test_ttl_entry_ord() {
        let entry_0 = TtlEntry {
            id: "0x1",
            ttl: 0,
            data: 0,
        };
        let entry_1 = TtlEntry {
            id: "0x1",
            ttl: 1,
            data: 0,
        };
        let entry_2 = TtlEntry {
            id: "0x2",
            ttl: 2,
            data: 0,
        };

        assert_eq!(Less, entry_0.cmp(&entry_1));
        assert_eq!(Less, entry_0.cmp(&entry_2));
        assert_eq!(Less, entry_1.cmp(&entry_2));

        assert_eq!(Greater, entry_1.cmp(&entry_0));
        assert_eq!(Greater, entry_2.cmp(&entry_1));
        assert_eq!(Greater, entry_2.cmp(&entry_1));

        assert_eq!(Equal, entry_0.cmp(&entry_0));
        assert_eq!(Equal, entry_1.cmp(&entry_1));
        assert_eq!(Equal, entry_2.cmp(&entry_2));
    }

    #[test]
    fn test_ttl_cache_insert_remove() {
        let ttl = Duration::seconds(1).whole_microseconds();
        let entry_1 = TtlEntry::new("0x1".to_string(), ttl, 0);
        let entry_2 = TtlEntry::new("0x2".to_string(), ttl, 0);

        let mut cache = TtlCache::new();

        cache.insert(entry_1.clone());

        assert!(cache.contains(&entry_1.id));
        assert!(!cache.contains(&entry_2.id));

        assert!(cache.remove(&entry_1));

        assert!(!cache.contains(&entry_1.id));
        assert!(!cache.contains(&entry_2.id));
    }

    #[test]
    fn test_ttl_cache_insert_get() {
        let ttl = Duration::seconds(1).whole_microseconds();
        let entry_1 = TtlEntry::new("0x1".to_string(), ttl, 0);

        let mut cache = TtlCache::new();

        cache.insert(entry_1.clone());

        assert!(cache.contains(&entry_1.id));

        let result = cache.get(&entry_1.id);
        assert!(result.is_some());
        assert_eq!(entry_1, *result.unwrap())
    }

    #[test]
    fn test_ttl_cache_expiry() {
        let entry_1 = TtlEntry::new(
            "0x1".to_string(),
            Duration::milliseconds(250).whole_microseconds(),
            "",
        );
        let entry_2 = TtlEntry::new(
            "0x2".to_string(),
            Duration::milliseconds(500).whole_microseconds(),
            "",
        );

        let mut cache = TtlCache::new();

        cache.insert(entry_1.clone());
        cache.insert(entry_2.clone());

        assert!(cache.contains(&entry_1.id));
        assert!(cache.contains(&entry_2.id));

        // let first entry expire
        sleep(StdDuration::from_millis(300));
        assert!(!cache.contains(&entry_1.id));
        assert!(cache.contains(&entry_2.id));

        // let second entry expire
        sleep(StdDuration::from_millis(300));
        assert!(!cache.contains(&entry_1.id));
        assert!(!cache.contains(&entry_2.id));
    }
}
