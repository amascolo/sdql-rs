use crate::Bool;
use approx::AbsDiffEq;
use hashbrown::hash_map::rayon::IntoParIter;
use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelIterator};
use std::fmt;
use std::hash::Hash;
use std::ops::{AddAssign, Deref, DerefMut, Index, IndexMut};

pub type HashSet<T> = HashMap<T, Bool>;

#[derive(Clone, Default, PartialEq)]
pub struct HashMap<K, V>(hashbrown::HashMap<K, V>)
where
    K: Eq + Hash;

impl<K, V> fmt::Display for HashMap<K, V>
where
    K: fmt::Display + Ord + Eq + Hash,
    V: fmt::Display + Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut entries: Vec<_> = self.iter().collect();
        entries.sort();
        let entries: Vec<_> = entries
            .into_iter()
            .map(|(k, v)| format!("{k} -> {v}"))
            .collect();
        write!(f, "{{\n    {}\n}}", entries.join(",\n    "))
    }
}
// TODO most likely get rid of this
impl<K, V> fmt::Debug for HashMap<K, V>
where
    K: fmt::Display + Ord + Eq + Hash,
    V: fmt::Display + Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

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

impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn from(arr: [(K, V); N]) -> Self {
        Self(hashbrown::HashMap::from(arr))
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

impl<K, V> AbsDiffEq for HashMap<K, V>
where
    K: AbsDiffEq + Ord + Eq + Hash,
    V: AbsDiffEq + Ord,
    K::Epsilon: Copy,
    V::Epsilon: Copy,
{
    type Epsilon = (K::Epsilon, V::Epsilon);

    fn default_epsilon() -> Self::Epsilon {
        (K::default_epsilon(), V::default_epsilon())
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let (k_eps, v_eps) = epsilon;

        let mut self_items: Vec<_> = self.iter().collect();
        let mut other_items: Vec<_> = other.iter().collect();
        self_items.sort();
        other_items.sort();

        Iterator::zip(self_items.iter(), other_items.iter())
            .all(|((k1, v1), (k2, v2))| k1.abs_diff_eq(k2, k_eps) && v1.abs_diff_eq(v2, v_eps))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_assign() {
        let mut map = HashMap::from([((), 0)]);
        assert_eq!(map[&()], 0);
        map[&()] += 1;
        assert_eq!(map[&()], 1);
    }
}
