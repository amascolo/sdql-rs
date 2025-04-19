use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::{
    cell::RefCell,
    ops::{AddAssign, Index, IndexMut},
    rc::Rc,
};

pub struct VecDict<T> {
    vec: Rc<RefCell<Vec<T>>>,
    proxy: Proxy<T>,
}

#[derive(Debug)]
pub struct Proxy<T> {
    vec: Rc<RefCell<Vec<T>>>,
    key: RefCell<Option<T>>,
}

impl<T> VecDict<T> {
    pub fn new(vec: Vec<T>) -> Self {
        let vec = Rc::new(RefCell::new(vec));
        let proxy = Proxy {
            vec: vec.clone(),
            key: RefCell::new(None),
        };
        Self { vec, proxy }
    }

    pub fn len(&self) -> usize {
        self.vec.borrow().len()
    }
}

impl<T> Default for VecDict<T> {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl<T, U> From<U> for VecDict<T>
where
    U: Into<Vec<T>>,
{
    fn from(value: U) -> Self {
        Self::new(value.into())
    }
}

impl<T> Index<T> for VecDict<T> {
    type Output = Proxy<T>;

    fn index(&self, _index: T) -> &Self::Output {
        unimplemented!()
    }
}

impl<T> IndexMut<T> for VecDict<T> {
    fn index_mut(&mut self, index: T) -> &mut Proxy<T> {
        self.proxy.key.replace(Some(index));
        &mut self.proxy
    }
}

impl<T, U> AddAssign<U> for Proxy<T> {
    fn add_assign(&mut self, _rhs: U) {
        let key = self.key.take().unwrap();
        self.vec.borrow_mut().push(key);
    }
}

impl<T> IntoIterator for VecDict<T>
where
    T: Debug,
{
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let Self { vec, proxy } = self;
        drop(proxy);
        Rc::try_unwrap(vec).unwrap().into_inner().into_iter()
    }
}

impl<T> Clone for VecDict<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.vec.borrow().clone())
    }
}

impl<T> PartialEq for VecDict<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.vec.borrow() == *other.vec.borrow()
    }
}

impl<T> Debug for VecDict<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VecDict")
            .field("vec", &self.vec)
            .field("proxy", &self.proxy)
            .finish()
    }
}

impl<T> Display for VecDict<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.vec.borrow())
    }
}

impl<T> Serialize for VecDict<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.vec.borrow().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for VecDict<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<T>::deserialize(deserializer)?;
        Ok(Self::new(vec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn add_assign() {
        let mut vd = VecDict::default();
        vd[()] += 1;
        assert_eq!(vd, VecDict::new(vec![()]));
        vd[()] += 2;
        assert_eq!(vd, VecDict::new(vec![(), ()]));
        vd[()] += ();
        assert_eq!(vd, VecDict::new(vec![(), (), ()]));
    }

    #[test]
    fn into_iter() {
        let vec = vec![1, 2, 3];
        let vd = VecDict::new(vec.clone());
        assert_equal(vd.into_iter(), vec.into_iter());
    }
}
