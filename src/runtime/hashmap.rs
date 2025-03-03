use derive_more::Display;
use hashbrown::hash_map::rayon::IntoParIter;
use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelIterator};
use std::hash::Hash;
use std::ops::{AddAssign, Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug, Display, Default, PartialEq)]
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

impl<K, V> HashMap<K, V>
where
    K: Copy + Eq + Hash,
    V: AddAssign + Default,
{
    pub fn sum(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<K, V> AddAssign for HashMap<K, V>
where
    K: Copy + Eq + Hash,
    V: AddAssign + Default,
{
    fn add_assign(&mut self, other: Self) {
        for (key, val) in other {
            self[&key] += val;
        }
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

impl<K, V> IntoIterator for HashMap<K, V>
where
    K: Eq + Hash,
{
    type Item = (K, V);
    type IntoIter = hashbrown::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        HashMap(iter.into_iter().collect())
    }
}

impl<K, V> IntoParallelIterator for HashMap<K, V>
where
    K: Eq + Hash + Send,
    V: Send,
{
    type Iter = IntoParIter<K, V>;
    type Item = (K, V);

    fn into_par_iter(self) -> Self::Iter {
        self.0.into_par_iter()
    }
}

impl<K, V> FromParallelIterator<(K, V)> for HashMap<K, V>
where
    K: Eq + Hash + Send,
    V: Send,
{
    fn from_par_iter<T: IntoParallelIterator<Item = (K, V)>>(iter: T) -> Self {
        HashMap(iter.into_par_iter().collect())
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
