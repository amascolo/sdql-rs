use std::marker::ConstParamTy;

#[derive(PartialEq, Eq, ConstParamTy)]
pub enum Addition {
    Plus,
    Min,
    Max,
}

// TODO semiring logic should be modelled on scalars, not hashmap
//  then we overload AddAssign += and MulAssign *= on the scalars
//
// pub struct Promote<T, const SR: SemiRing = { SemiRing::SumProduct }>(T)
// where
//     T: AddAssign + MulAssign + Ord;
//
// // see table, pg. 5 https://dl.acm.org/doi/10.1145/3527333
// #[derive(PartialEq, Eq, ConstParamTy)]
// pub enum SemiRing {
//     SumProduct,
//     MinProduct, // mnpr
//     MaxProduct, // mxpr
//     MinSum,     // mnsm
//     MaxSum,     // mxsm
//     MaxMin,     // mxmn
// }
//
// // TODO macros e.g. mnpr!(x) as Promo::<_, SemiRing::MinProduct>(x)
