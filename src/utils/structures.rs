use std::collections::HashMap;
use std::fmt;

pub struct Headers {
    hash: HashMap<String, String>
}

impl Headers {
    pub fn new() -> Headers {
        Headers {
            hash: HashMap::new()
        }
    }

    pub fn update(mut self, hash: HashMap<String, String>) {
        let hash_as_vec: Vec<(String, String)> = hash.iter().map(|(x, y)| (x.clone(), y.clone())).collect();
        for (key, value) in hash_as_vec {
            self.hash.insert(key.to_lowercase(), value);
        }
    }

    pub fn from_hash(hash: HashMap<String, String>) -> Headers {
        Headers {
            hash: hash
        }
    }

    pub fn insert(mut self, key: String, value: String) {
        self.hash.insert(key.to_lowercase(), value);
    }

    pub fn remove(mut self, key: String) {
        self.hash.remove(&key.to_lowercase());
    }

    pub fn contains_key(self, key: String) -> bool {
        self.hash.contains_key(&key.to_lowercase())
    }

    pub fn keys(self) -> Vec<String> {
        self.hash.keys().cloned().collect()
    }
    
    pub fn values(self) -> Vec<String> {
        self.hash.values().cloned().collect()
    }

    pub fn iter(self) -> Vec<(String, String)> {
        self.hash.iter().map(|(x, y)| (x.clone(), y.clone())).collect()
    }

    pub fn clear(mut self) {
        self.hash.clear();
    }

    pub fn as_string(&self) -> String {
        let vec_of_strings: Vec<String> = self.hash.iter().map(|(x, y)| format!("{}: {}", x, y)).collect();
        vec_of_strings.join("\r\n")
    }
}

impl fmt::Display for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}