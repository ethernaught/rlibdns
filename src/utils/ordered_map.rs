use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct OrderedMap<K: Eq + Hash, V> {
    map: HashMap<K, V>,
    keys: Vec<K>,
}

impl<K, V> OrderedMap<K, V>
where
    K: Eq + Hash + Clone
{

    pub fn new() -> Self {
        OrderedMap {
            map: HashMap::new(),
            keys: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value)
        } else {
            self.keys.push(key.clone());
            self.map.insert(key, value)
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.map.remove(key) {
            self.keys.retain(|k| k != key);
            Some(value)
        } else {
            None
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.get_mut(key)
    }

    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        match self.map.entry(key.clone()) {
            Entry::Occupied(occupied) => {
                Entry::Occupied(occupied)
            }
            Entry::Vacant(vacant) => {
                self.keys.push(key.clone());
                Entry::Vacant(vacant)
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys.iter().filter_map(move |key| {
            let value = self.map.get(key)?;
            Some((key, value))
        })
    }

    pub fn keys(&self) -> &Vec<K> {
        &self.keys
    }
    
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.map.values()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn drain(&mut self) -> impl Iterator<Item = (K, V)> + use<'_, K, V> {
        self.keys
            .drain(..)
            .filter_map(|key| self.map.remove(&key).map(|v| (key, v)))
    }
}
