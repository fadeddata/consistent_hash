use std::fmt::Debug;
use std::collections::BTreeMap;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct ConsistentHash<T: Hash> {
    virtuals: isize,
    ring: BTreeMap<String, T>
}

impl<T: Hash+Clone+Debug> ConsistentHash<T> {
	pub fn new(nodes: Vec<T>, virtuals: isize) -> ConsistentHash<T> {
        let mut new_consistent_hash = ConsistentHash {
            virtuals: virtuals,
            ring: BTreeMap::new()
        };

        for node in nodes {
            new_consistent_hash.insert(node);
        }

        new_consistent_hash
    }

    pub fn insert(&mut self, node: T) {
        for i in 0..self.virtuals {
            let hash = calculate_hash(&node);
            let key = gen_key(format!("{}:{}", i, hash));
            self.ring.insert(key, node.clone());
        }
    }

    pub fn remove(&mut self, node: T) {
        for i in 0..self.virtuals {
            let hash = calculate_hash(&node);
            let key = gen_key(format!("{}:{}", i, hash));
            self.ring.remove(&key);
        }
    }

    pub fn get<K: Hash>(&mut self, key: K) -> Option<&T> {
        if self.ring.len() == 0 {
            return None;
        }

        let hash = calculate_hash(&key);
        let key = gen_key(format!("{}", hash));

        let sorted_keys: Vec<&String> = self.ring.keys().collect();
        let head_key = sorted_keys[0].clone();

        for k in sorted_keys {
            if k >= &key {
                return self.ring.get(k);
            } 
        }

        self.ring.get(&head_key)
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn gen_key(key: String) -> String {
    let mut sh = Md5::new();
    sh.input_str(&key);
    sh.result_str()
}

#[cfg(test)]
mod test {
	use consistent_hash::ConsistentHash;

	#[test]
	fn test_empty() {
		let mut consistent_hash: ConsistentHash<String> = ConsistentHash::new(vec![], 10);
		assert_eq!(None, consistent_hash.get("foo".to_string()));
	}

	#[test]
    fn test_full() {
    	let mut consistent_hash: ConsistentHash<String> = ConsistentHash::new(vec!["foo".to_string(), "bar".to_string(), "baz".to_string()], 10);

        assert_eq!(Some("foo".to_string()), consistent_hash.get("1").map(|x| x.to_string()));
        assert_eq!(Some("baz".to_string()), consistent_hash.get("2").map(|x| x.to_string()));
        assert_eq!(Some("baz".to_string()), consistent_hash.get("3").map(|x| x.to_string()));
        assert_eq!(Some("bar".to_string()), consistent_hash.get("4").map(|x| x.to_string()));
        assert_eq!(Some("foo".to_string()), consistent_hash.get("5").map(|x| x.to_string()));
        assert_eq!(Some("bar".to_string()), consistent_hash.get("6").map(|x| x.to_string()));
        assert_eq!(Some("bar".to_string()), consistent_hash.get("7").map(|x| x.to_string()));
        assert_eq!(Some("foo".to_string()), consistent_hash.get("8").map(|x| x.to_string()));
        assert_eq!(Some("baz".to_string()), consistent_hash.get("9").map(|x| x.to_string()));

        consistent_hash.remove("foo".to_string());

        assert_eq!(Some("bar".to_string()), consistent_hash.get("1").map(|x| x.to_string()));
        assert_eq!(Some("bar".to_string()), consistent_hash.get("5").map(|x| x.to_string()));
        assert_eq!(Some("baz".to_string()), consistent_hash.get("8").map(|x| x.to_string()));

        consistent_hash.insert("foo".to_string());

        assert_eq!(Some("foo".to_string()), consistent_hash.get("1").map(|x| x.to_string()));
        assert_eq!(Some("foo".to_string()), consistent_hash.get("5").map(|x| x.to_string()));
        assert_eq!(Some("foo".to_string()), consistent_hash.get("8").map(|x| x.to_string()));
    }
}