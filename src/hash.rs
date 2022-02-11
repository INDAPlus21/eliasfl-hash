use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
};

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    count: usize,
}

pub struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            buckets: Vec::with_capacity(100),
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
        let mut hasher = PrimeHasher::new(0);
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

    pub fn insert(&self, key: K, val: V) {
        todo!("resize if some percentage of full")
    }
}

struct PrimeHasher {
    state: u64,
}
impl PrimeHasher {
    fn new(state: u64) -> Self {
        Self { state }
    }
}

impl Hasher for PrimeHasher {
    fn finish(&self) -> u64 {
        self.state
    }
    fn write(&mut self, bytes: &[u8]) {
        for i in bytes {
            self.write_u8(*i);
        }
    }

    fn write_u8(&mut self, i: u8) {
        self.state = (13 * self.state + i as u64) % u8::MAX as u64;
    }
    fn write_u16(&mut self, i: u16) {
        self.state = (13 * self.state + i as u64) % u16::MAX as u64;
    }
    fn write_u32(&mut self, i: u32) {
        self.state = (13 * self.state + i as u64) % u32::MAX as u64;
    }
    fn write_u64(&mut self, i: u64) {
        self.state = (13 * self.state + i as u64) % u64::MAX as u64;
    }
    fn write_usize(&mut self, i: usize) {
        self.state = (13 * self.state + i as u64) % usize::MAX as u64;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_string() {
        let mut hasher = PrimeHasher::new(0);
        hasher.write("lorem ipsum".as_bytes());
        let hash = hasher.finish();
        println!("{}", hash);
    }
}
