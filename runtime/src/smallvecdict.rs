use smallvec::{Array, SmallVec};
use std::fmt::Debug;
use std::{
    cell::RefCell,
    ops::{AddAssign, Index, IndexMut},
    rc::Rc,
};

#[derive(Debug, PartialEq)]
pub struct SmallVecDict<T>
where
    T: Array,
    <T as Array>::Item: Debug + PartialEq,
{
    vec: Rc<RefCell<SmallVec<T>>>,
    proxy: Proxy<T>,
}

#[derive(Debug, PartialEq)]
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
    <T as Array>::Item: Debug + PartialEq,
{
    pub fn new(vec: SmallVec<T>) -> Self {
        let vec = Rc::new(RefCell::new(vec));
        let proxy = Proxy {
            vec: vec.clone(),
            key: RefCell::new(None),
        };
        SmallVecDict { vec, proxy }
    }
}

impl<T, U> Index<U> for SmallVecDict<T>
where
    T: Array<Item = U>,
    <T as Array>::Item: Debug + PartialEq,
{
    type Output = Proxy<T>;

    fn index(&self, _index: U) -> &Self::Output {
        unimplemented!()
    }
}

impl<T, U> IndexMut<U> for SmallVecDict<T>
where
    T: Array<Item = U>,
    <T as Array>::Item: Debug + PartialEq,
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
    <T as Array>::Item: Debug + PartialEq,
{
    type Item = T::Item;
    type IntoIter = <SmallVec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let SmallVecDict { vec, proxy } = self;
        drop(proxy);
        Rc::try_unwrap(vec).unwrap().into_inner().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;
    use smallvec::smallvec;

    #[test]
    fn add_assign() {
        let mut vd: SmallVecDict<[_; 4]> = SmallVecDict::new(smallvec![]);
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
