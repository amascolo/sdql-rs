use sdql::backend::fmf::ExprFMF;
use sdql::frontend::lexer::Spanned;
use sdql::inference::Typed;
use sdql::ir::expr::Expr;
use sdql::sdql;

fn e2e(src: &str) {
    let expr = sdql!(src);
    let typed = Typed::from(expr);
    let fmf = ExprFMF::from(typed);
    println!("{fmf}");
    // let string: String = sdql!(src).into();
    // println!("{string}");
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
