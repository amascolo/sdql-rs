use sdql::rs;

#[test]
fn tpch_q3() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q3.sdql"));
    let _ = rs!(src);
}

#[test]
fn tpch_q6() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q6.sdql"));
    let _ = rs!(src);
}
