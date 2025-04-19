use serde::{Deserialize, Deserializer, Serialize, Serializer};
use smallvec::{smallvec, Array, SmallVec};
use std::fmt::{Debug, Display, Formatter};
use std::{
    cell::RefCell,
    ops::{AddAssign, Index, IndexMut},
    rc::Rc,
};

pub struct SmallVecDict<T>
where
    T: Array,
{
    vec: Rc<RefCell<SmallVec<T>>>,
    proxy: Proxy<T>,
}

#[derive(Debug)]
pub struct Proxy<T>
where
    T: Array,
{
    vec: Rc<RefCell<SmallVec<T>>>,
    key: RefCell<Option<T::Item>>,
}

impl<T> SmallVecDict<T>
where
    T: Array,
{
    pub fn new(vec: SmallVec<T>) -> Self {
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

impl<T> Default for SmallVecDict<T>
where
    T: Array,
{
    fn default() -> Self {
        Self::new(smallvec![])
    }
}

impl<T, U> From<U> for SmallVecDict<T>
where
    T: Array,
    U: Into<SmallVec<T>>,
{
    fn from(value: U) -> Self {
        Self::new(value.into())
    }
}

impl<T, U> Index<U> for SmallVecDict<T>
where
    T: Array<Item = U>,
{
    type Output = Proxy<T>;

    fn index(&self, _index: U) -> &Self::Output {
        unimplemented!()
    }
}

impl<T, U> IndexMut<U> for SmallVecDict<T>
where
    T: Array<Item = U>,
{
    fn index_mut(&mut self, index: U) -> &mut Proxy<T> {
        self.proxy.key.replace(Some(index));
        &mut self.proxy
    }
}

impl<T, U> AddAssign<U> for Proxy<T>
where
    T: Array,
{
    fn add_assign(&mut self, _rhs: U) {
        let key = self.key.take().unwrap();
        self.vec.borrow_mut().push(key);
    }
}

impl<T> IntoIterator for SmallVecDict<T>
where
    T: Array,
    <T as Array>::Item: Debug,
{
    type Item = T::Item;
    type IntoIter = <SmallVec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let SmallVecDict { vec, proxy } = self;
        drop(proxy);
        Rc::try_unwrap(vec).unwrap().into_inner().into_iter()
    }
}

impl<T> Clone for SmallVecDict<T>
where
    T: Array,
    <T as Array>::Item: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.vec.borrow().clone())
    }
}

impl<T> PartialEq for SmallVecDict<T>
where
    T: Array,
    <T as Array>::Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.vec.borrow() == *other.vec.borrow()
    }
}

impl<T> Eq for SmallVecDict<T>
where
    T: Array,
    <T as Array>::Item: Eq,
{
}

impl<T> Debug for SmallVecDict<T>
where
    T: Array + Debug,
    <T as Array>::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmallVecDict")
            .field("vec", &self.vec)
            .field("proxy", &self.proxy)
            .finish()
    }
}

impl<T> Display for SmallVecDict<T>
where
    T: Array,
    <T as Array>::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.vec.borrow())
    }
}

impl<T> Serialize for SmallVecDict<T>
where
    T: Array,
    T::Item: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.vec.borrow().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for SmallVecDict<T>
where
    T: Array,
    T::Item: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = SmallVec::<T>::deserialize(deserializer)?;
        Ok(Self::new(vec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;
    use smallvec::smallvec;

    #[test]
    fn add_assign() {
        let mut vd: SmallVecDict<[_; 4]> = SmallVecDict::default();
        vd[()] += 1;
        assert_eq!(vd, SmallVecDict::new(smallvec![()]));
        vd[()] += 2;
        assert_eq!(vd, SmallVecDict::new(smallvec![(), ()]));
        vd[()] += ();
        assert_eq!(vd, SmallVecDict::new(smallvec![(), (), ()]));
    }

    #[test]
    fn into_iter() {
        let vec = smallvec![1, 2, 3];
        let vd: SmallVecDict<[_; 4]> = SmallVecDict::new(vec.clone());
        assert_equal(vd.into_iter(), vec.into_iter());
    }
}
