use sdql::backend::fmf::ExprFMF;
use sdql::frontend::lexer::Spanned;
use sdql::inference::{Typed, TypedExpr};
use sdql::ir::expr::Expr;
use sdql::sdql;

// TODO one liner
fn e2e(src: &str) {
    let expr = sdql!(src);
    // println!("{expr:#?}");
    let typed: Typed<Spanned<TypedExpr>> = Typed::from(expr);
    // println!("{typed}");
    let fmf = Typed::<Spanned<ExprFMF>>::from(typed);
    // println!("{fmf}");
    let string = String::from(fmf);
    println!("{string}");
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
}
