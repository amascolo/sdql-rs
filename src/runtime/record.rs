use derive_more::Display;
use std::ops::AddAssign;

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{_0:?}")]
pub struct Record<T: IsTuple>(T);

pub trait IsTuple {}
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

macro_rules! impl_add_assign {
    () => {
        impl AddAssign for Record<()> {
            fn add_assign(&mut self, _rhs: Self) {}
        }
    };

    ($($T:ident),*; $($idx:tt),*) => {
        impl<$($T),*> AddAssign for Record<($($T,)*)>
        where
            $($T: AddAssign),*
        {
            fn add_assign(&mut self, rhs: Self) {
                $(self.0.$idx += rhs.0.$idx;)*
            }
        }
    };
}

impl_add_assign!();
impl_add_assign!(T1; 0);
impl_add_assign!(T1, T2; 0, 1);
impl_add_assign!(T1, T2, T3; 0, 1, 2);
impl_add_assign!(T1, T2, T3, T4; 0, 1, 2, 3);
impl_add_assign!(T1, T2, T3, T4, T5; 0, 1, 2, 3, 4);
impl_add_assign!(T1, T2, T3, T4, T5, T6; 0, 1, 2, 3, 4, 5);
impl_add_assign!(T1, T2, T3, T4, T5, T6, T7; 0, 1, 2, 3, 4, 5, 6);
impl_add_assign!(T1, T2, T3, T4, T5, T6, T7, T8; 0, 1, 2, 3, 4, 5, 6, 7);
impl_add_assign!(T1, T2, T3, T4, T5, T6, T7, T8, T9; 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_add_assign!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_add_assign!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_add_assign!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

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

    #[test]
    fn add_assign() {
        let mut rec = Record(());
        rec += Record(());
        assert_eq!(rec, Record(()));

        let mut rec = Record((0,));
        rec += Record((1,));
        assert_eq!(rec, Record((1,)));

        let mut rec = Record((0, 0));
        rec += Record((1, 1));
        assert_eq!(rec, Record((1, 1)));

        let mut rec = Record((0, 0, 0));
        rec += Record((1, 1, 1));
        assert_eq!(rec, Record((1, 1, 1)));

        let mut rec = Record((0, 0, 0, 0));
        rec += Record((1, 1, 1, 1));
        assert_eq!(rec, Record((1, 1, 1, 1)));

        let mut rec = Record((0, 0, 0, 0, 0));
        rec += Record((1, 1, 1, 1, 1));
        assert_eq!(rec, Record((1, 1, 1, 1, 1)));

        let mut rec = Record((0, 0, 0, 0, 0, 0));
        rec += Record((1, 1, 1, 1, 1, 1));
        assert_eq!(rec, Record((1, 1, 1, 1, 1, 1)));

        let mut rec = Record((0, 0, 0, 0, 0, 0, 0));
        rec += Record((1, 1, 1, 1, 1, 1, 1));
        assert_eq!(rec, Record((1, 1, 1, 1, 1, 1, 1)));

        let mut rec = Record((0, 0, 0, 0, 0, 0, 0, 0));
        rec += Record((1, 1, 1, 1, 1, 1, 1, 1));
        assert_eq!(rec, Record((1, 1, 1, 1, 1, 1, 1, 1)));

        let mut rec = Record((0, 0, 0, 0, 0, 0, 0, 0, 0));
        rec += Record((1, 1, 1, 1, 1, 1, 1, 1, 1));
        assert_eq!(rec, Record((1, 1, 1, 1, 1, 1, 1, 1, 1)));

        let mut rec = Record((0, 0, 0, 0, 0, 0, 0, 0, 0, 0));
        rec += Record((1, 1, 1, 1, 1, 1, 1, 1, 1, 1));
        assert_eq!(rec, Record((1, 1, 1, 1, 1, 1, 1, 1, 1, 1)));

        let mut rec = Record((0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0));
        rec += Record((1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1));
        assert_eq!(rec, Record((1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1)));

        let mut rec = Record((0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0));
        rec += Record((1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1));
        assert_eq!(rec, Record((1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1)));
    }
}
