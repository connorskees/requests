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

    pub fn update(mut self, hash: HashMap<String, String>) -> Headers {
        let hash_as_vec: Vec<(String, String)> = hash.iter().map(|(x, y)| (x.clone(), y.clone())).collect();
        for (key, value) in hash_as_vec {
            self.hash.insert(key.to_lowercase(), value);
        }
        self
    }

    pub fn update_from_headers(self, hash: Headers) -> Headers {
        self.update(hash.as_hash())
    }

    pub fn from_hash(hash: HashMap<String, String>) -> Headers {
        let mut h: HashMap<String, String> = HashMap::new();
        for (key, value) in hash.iter() {
            h.insert(String::from(key).to_lowercase(), String::from(value));
        }
        Headers {
            hash: h
        }
    }

    pub fn as_hash(self) -> HashMap<String, String> {
        self.hash
    }

    pub fn insert(&mut self, key: String, value: String) {
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

    pub fn as_string(&self, delim: &str) -> String {
        let vec_of_strings: Vec<String> = self.hash.iter().map(|(x, y)| format!("{}: {}", x, y)).collect();
        vec_of_strings.join(delim)
    }

    pub fn get(&self, key: String) -> Option<&String>  {
        self.hash.get(&key.to_lowercase())
    }
}

impl fmt::Display for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string("\r\n"))
    }
}

impl fmt::Debug for Headers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Headers {{\n\t{}\n}}", self.as_string("\n\t"))
    }
}