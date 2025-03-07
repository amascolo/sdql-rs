use sdql::backend::fmf::ExprFMF;
use sdql::frontend::lexer::Spanned;
use sdql::inference::{Typed, TypedExpr};
use sdql::ir::expr::Expr;

fn e2e(src: &str) {
    let expr = Spanned::<Expr>::try_from(src).unwrap();
    let t: Typed<Spanned<TypedExpr>> = expr.into();
    let fmf: ExprFMF = t.into();
    println!("{fmf}");
}

#[test]
fn tpch_q3() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q3.sdql"));
    e2e(src);
}

#[test]
fn tpch_q6() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q6.sdql"));
    e2e(src);
    // TODO
    // let s: String = sdql!(src).into();
    // println!("{s}");
}
