use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::mem;

const INITIAL_BUCKETS: usize = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    count: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            count: 0,
        }
    }

    fn get_bucket<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.buckets.len() == 0 {
            return None;
        }
        let mut hasher = SdbmHasher::new(0);
        key.hash(&mut hasher);
        let hash = hasher.finish() % self.buckets.len() as u64;
        Some(hash as usize)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.get_bucket(key)?;
        for (key_, val_) in &self.buckets[bucket] {
            if key == key_.borrow() {
                return Some(val_);
            }
        }
        None
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        // If empty or 3/4 full -> resize
        if self.buckets.is_empty() || self.count > 3 * self.buckets.len() / 4 {
            self.resize()
        }

        let index = self.get_bucket(&key)?;
        let bucket = &mut self.buckets[index];

        for &mut (ref key_, ref mut val_) in bucket.iter_mut() {
            if key_ == &key {
                return Some(mem::replace(val_, val));
            }
        }

        self.count += 1;
        bucket.push((key, val));
        None
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = self.get_bucket(key)?;
        let bucket = &mut self.buckets[index];
        let i = bucket.iter().position(|&(ref e, _)| e.borrow() == key)?;
        self.count -= 1;
        Some(bucket.swap_remove(i).1)
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_BUCKETS, // Initial buckets
            n => 2 * n,
        };
        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = SdbmHasher::new(0);
            key.hash(&mut hasher);
            let hash = hasher.finish() % new_buckets.len() as u64;
            let bucket = hash as usize;
            new_buckets[bucket].push((key, value));
        }

        let _ = mem::replace(&mut self.buckets, new_buckets);
    }

    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

pub struct HashMapIter<'a, K: 'a, V: 'a> {
    map: &'a HashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> Iterator for HashMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => {
                    match bucket.get(self.at) {
                        Some(&(ref k, ref v)) => {
                            // move along self.at and self.bucket
                            self.at += 1;
                            break Some((k, v));
                        }
                        None => {
                            self.bucket += 1;
                            self.at = 0;
                            continue;
                        }
                    }
                }
                None => break None,
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = HashMapIter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        HashMapIter {
            map: self,
            bucket: 0,
            at: 0,
        }
    }
}

pub struct IntoIter<K, V> {
    map: HashMap<K, V>,
    bucket: usize,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get_mut(self.bucket) {
                Some(bucket) => match bucket.pop() {
                    Some(x) => break Some(x),
                    None => {
                        self.bucket += 1;
                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<K, V> IntoIterator for HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            map: self,
            bucket: 0,
        }
    }
}

use std::iter::FromIterator;
impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut map = HashMap::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}

pub struct SdbmHasher {
    state: u64,
}
impl SdbmHasher {
    pub fn new(state: u64) -> Self {
        Self { state }
    }

    /// sdbm Hashing algorithm
    pub fn hash_int<T: Into<u64>>(&mut self, number: T) {
        self.state = number
            .into()
            .wrapping_add(self.state << 6)
            .wrapping_add(self.state << 16)
            .wrapping_sub(self.state);
    }

    /// Hash a string slice
    pub fn hash_string(&mut self, string: &str) {
        for num in string.encode_utf16() {
            self.hash_int(num);
        }
    }
}
impl Hasher for SdbmHasher {
    fn finish(&self) -> u64 {
        self.state
    }
    fn write(&mut self, bytes: &[u8]) {
        for i in bytes {
            self.write_u8(*i);
        }
    }

    fn write_u8(&mut self, i: u8) {
        self.hash_int(i);
    }
    fn write_u16(&mut self, i: u16) {
        self.hash_int(i);
    }
    fn write_u32(&mut self, i: u32) {
        self.hash_int(i);
    }
    fn write_u64(&mut self, i: u64) {
        self.hash_int(i);
    }
    fn write_usize(&mut self, i: usize) {
        self.hash_int(i as u64);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashmap() {
        let mut map = HashMap::<u64, String>::new();
        map.insert(123, "text123string".to_owned());
        map.insert(123, "new string".to_owned());
        map.insert(5324, "lorem ipsum".to_owned());
        println!("{:?}", map.buckets.iter().collect::<Vec<_>>());
        assert_eq!(map.get(&123), Some(&"new string".to_owned()));
        assert_eq!(map.get(&1234), None);
    }

    #[test]
    fn test_hash_string() {
        let hashes = [
            "",
            "a",
            "aa",
            "aaaaaaaa",
            "lorem ipsum",
            "lorem ipsun",
            "lorem ppsum",
            "test123",
        ]
        .map(|string| {
            let mut hasher = SdbmHasher::new(0);
            hasher.write(string.as_bytes());
            hasher.finish()
        });
        assert_eq!(
            hashes,
            [
                0,
                97,
                6363200,
                3213702187780907264,
                13432600051413073019,
                13432600051413073020,
                13929838331737494914,
                15071901698212411520
            ]
        );
    }

    /// Modified test from https://www.programmingalgorithms.com/algorithm/sdbm-hash?lang=C%2B%2B
    #[test]
    fn test_predefined_hash() {
        let string = "jdfgsdhfsdfsd 6445dsfsd7fg/*/+bfjsdgf%$^";
        let mut hasher = SdbmHasher::new(0);
        hasher.hash_string(string);
        let hash = hasher.finish();
        assert_eq!(hash, 11139173862158291091);
    }
}
