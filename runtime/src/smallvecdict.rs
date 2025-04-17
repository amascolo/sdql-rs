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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_assign() {
        let mut vd: SmallVecDict<[(); 4]> = SmallVecDict::new(vec![].into());
        vd[()] += 1;
        assert_eq!(vd, SmallVecDict::new(vec![()].into()));
    }
}
