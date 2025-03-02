#![allow(dead_code)]

use derive_more::Display;

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{_0:?}")]
struct Record<T: IsTuple>(pub T);

trait IsTuple {}
macro_rules! impl_is_tuple {
    ($($name:ident),*) => {
        impl<$($name),*> IsTuple for ($($name,)*) {}
    };
}
impl_is_tuple!();
impl_is_tuple!(T1);
impl_is_tuple!(T1, T2);
impl_is_tuple!(T1, T2, T3);
impl_is_tuple!(T1, T2, T3, T4);
impl_is_tuple!(T1, T2, T3, T4, T5);
impl_is_tuple!(T1, T2, T3, T4, T5, T6);
impl_is_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_is_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_is_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_is_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_is_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_is_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_tuple() {
        let _ = Record(());
        let _ = Record(((),));
        let _ = Record(((), ()));
        let _ = Record(((), (), ()));
        let _ = Record(((), (), (), ()));
        let _ = Record(((), (), (), (), ()));
        let _ = Record(((), (), (), (), (), ()));
        let _ = Record(((), (), (), (), (), (), ()));
        let _ = Record(((), (), (), (), (), (), (), ()));
        let _ = Record(((), (), (), (), (), (), (), (), ()));
        let _ = Record(((), (), (), (), (), (), (), (), (), ()));
        let _ = Record(((), (), (), (), (), (), (), (), (), (), ()));
        let _ = Record(((), (), (), (), (), (), (), (), (), (), (), ()));
    }
}
