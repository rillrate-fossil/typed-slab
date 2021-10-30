pub use slab::Slab;
use std::marker::PhantomData;

#[derive(Debug, Default)]
pub struct TypedSlab<K, V> {
    slab: Slab<V>,
    _key: PhantomData<K>,
}

impl<K, V> TypedSlab<K, V>
where
    K: From<usize> + Into<usize>,
{
    pub fn new() -> Self {
        Self {
            slab: Slab::new(),
            _key: PhantomData,
        }
    }

    pub fn insert(&mut self, value: V) -> K {
        let idx = self.slab.insert(value);
        K::from(idx)
    }

    pub fn insert_entry(&mut self, value: V) -> (K, &mut V) {
        let entry = self.slab.vacant_entry();
        let idx = entry.key();
        let value_mut = entry.insert(value);
        (K::from(idx), value_mut)
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let idx = key.into();
        if self.slab.contains(idx) {
            let value = self.slab.remove(idx);
            Some(value)
        } else {
            None
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let idx = key.into();
        self.slab.get(idx)
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let idx = key.into();
        self.slab.get_mut(idx)
    }

    pub fn is_empty(&self) -> bool {
        self.slab.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.slab.iter().map(|(idx, v)| (idx.into(), v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> {
        self.slab.iter_mut().map(|(idx, v)| (idx.into(), v))
    }

    pub fn len(&self) -> usize {
        self.slab.len()
    }
}
