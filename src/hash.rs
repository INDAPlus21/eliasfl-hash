pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    count: usize,
}

pub struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            count: 0,
        }
    }
}

pub fn hash_u32_vec(numbers: Vec<u32>, length: u32) -> u32 {
    let mut hash = 0;
    for num in numbers {
        hash = (13 * hash + num) % length;
    }
    hash
}

pub fn hash_string(string: &str, length: u32) -> u32 {
    hash_u32_vec(string.chars().map(|chr| chr as u32).collect(), length)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_string() {
        let hash = hash_string("lorem ipsum", 100);
        println!("{}", hash);
    }
}
