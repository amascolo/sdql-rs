use super::*;
use crate::sdql;

#[test]
fn tpch_q1() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/1.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q2() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/2.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q3() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/3.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q4() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/4.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q5() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/5.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q6() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/6.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q7() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/7.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q8() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/8.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q9() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/9.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q10() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/10.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q11() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/11.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q12() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/12.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q13() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/13.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q14() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/14.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

// FIXME TPCH q15 add support for max
// #[test]
// fn tpch_q15() {
//     let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/15.sdql"));
//     let expr = sdql!(src);
//     assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
// }

#[test]
fn tpch_q16() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/16.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q17() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/17.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q18() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/18.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q19() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/19.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q20() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/20.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q21() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/21.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}

#[test]
fn tpch_q22() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/22.sdql"));
    let expr = sdql!(src);
    assert_eq!(Spanned::from(Typed::from(expr.clone())), expr);
}
