use sdql::backend::ExprFMF;
use sdql::frontend::lexer::Spanned;
use sdql::inference::{Typed, TypedExpr};
use sdql::ir::expr::Expr;

#[test]
fn tpch_q6() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q6.sdql"));
    let expr = Spanned::<Expr>::try_from(src).unwrap();
    println!("{expr}");
    let t: Typed<Spanned<TypedExpr>> = expr.into();
    println!("{t:?}");
    let fmf: ExprFMF = t.into();
    println!("{fmf:?}");
}
