use approx::AbsDiffEq;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Deref};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct Record<T: IsTuple>(T);

impl<T> AbsDiffEq for Record<T>
where
    T: IsTuple + AbsDiffEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}

// TODO should print record names too
// TODO display inner tuple types, not debug
impl<T> fmt::Display for Record<T>
where
    T: IsTuple + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tuple = format!("{:?}", self.0);
        let angled = format!("<{}>", &tuple[1..tuple.len() - 1]);
        write!(f, "{angled}")
    }
}

impl<T> Record<T>
where
    T: IsTuple,
{
    pub const fn new(value: T) -> Self {
        Record(value)
    }

    pub fn decat(self) -> T {
        let Self(value) = self;
        value
    }
}

impl<T: IsTuple> Record<T> {}

impl<T> Deref for Record<T>
where
    T: IsTuple,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO we can make it variadic
impl<T> Sum for Record<(T,)>
where
    T: Sum,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Record((iter.map(|Record((t,))| t).sum(),))
    }
}
// TODO this was required for TPCH Q14 but we can make it variadic
impl<T, U> Sum for Record<(T, U)>
where
    T: Sum + Add<Output = T>,
    U: Sum + Add<Output = U>,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::new(iter.map(|Record((a, b))| (a, b)).fold(
            (T::sum(std::iter::empty()), U::sum(std::iter::empty())),
            |(acc_a, acc_b), (a, b)| (acc_a + a, acc_b + b),
        ))
    }
}
// TODO this was required for TPCH Q11 but we can make it variadic

pub trait IsTuple {}

macro_rules! impl_is_tuple {
    () => {
        impl IsTuple for () {}
    };

    ($($idx:tt),+) => {
        paste::paste! {
            impl<$( [<T $idx>], )+> IsTuple for ($( [<T $idx>], )+) {}
        }
    };
}

impl_is_tuple!();
impl_is_tuple!(0);
impl_is_tuple!(0, 1);
impl_is_tuple!(0, 1, 2);
impl_is_tuple!(0, 1, 2, 3);
impl_is_tuple!(0, 1, 2, 3, 4);
impl_is_tuple!(0, 1, 2, 3, 4, 5);
impl_is_tuple!(0, 1, 2, 3, 4, 5, 6);
impl_is_tuple!(0, 1, 2, 3, 4, 5, 6, 7);
impl_is_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_is_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_is_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_is_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
// note: we can create records this size, but underlying tuples won't have traits
impl_is_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
impl_is_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
impl_is_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
impl_is_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

macro_rules! impl_add_assign {
    () => {
        impl AddAssign for Record<()> {
            fn add_assign(&mut self, _rhs: Self) {}
        }
    };

    ($($idx:tt),+) => {
        paste::paste! {
            impl<$( [<T $idx>], )+> AddAssign for Record<($( [<T $idx>], )+)>
            where
                $( [<T $idx>]: AddAssign, )+
            {
                fn add_assign(&mut self, rhs: Self) {
                    $( self.0.$idx += rhs.0.$idx; )+
                }
            }
        }
    };
}

impl_add_assign!();
impl_add_assign!(0);
impl_add_assign!(0, 1);
impl_add_assign!(0, 1, 2);
impl_add_assign!(0, 1, 2, 3);
impl_add_assign!(0, 1, 2, 3, 4);
impl_add_assign!(0, 1, 2, 3, 4, 5);
impl_add_assign!(0, 1, 2, 3, 4, 5, 6);
impl_add_assign!(0, 1, 2, 3, 4, 5, 6, 7);
impl_add_assign!(0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_add_assign!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_add_assign!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_add_assign!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
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
        // note: we can create records this size, but underlying tuples won't have traits
        let _ = Record(((), (), (), (), (), (), (), (), (), (), (), (), ()));
        let _ = Record(((), (), (), (), (), (), (), (), (), (), (), (), (), ()));
        let _ = Record(((), (), (), (), (), (), (), (), (), (), (), (), (), (), ()));
        let _ = Record(((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()));
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
