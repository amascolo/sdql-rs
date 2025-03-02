use derive_more::Display;
use std::hash::Hash;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{_0}")]
pub struct HashMap<K, V>(hashbrown::HashMap<K, V>)
where
    K: Eq + Hash;

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        HashMap(hashbrown::HashMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        HashMap(hashbrown::HashMap::with_capacity(capacity))
    }
}

impl<K, V> Deref for HashMap<K, V>
where
    K: Eq + Hash,
{
    type Target = hashbrown::HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> Index<&K> for HashMap<K, V>
where
    K: Eq + Hash,
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        &self.0[key]
    }
}

impl<K, V> IndexMut<&K> for HashMap<K, V>
where
    K: Copy + Eq + Hash,
    V: Default,
{
    fn index_mut(&mut self, key: &K) -> &mut Self::Output {
        self.entry(*key).or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map: HashMap<(), u8> = HashMap::new();
        map.insert((), 0);
        assert_eq!(map[&()], 0);
        map[&()] += 1;
        assert_eq!(map[&()], 1);
    }
}
