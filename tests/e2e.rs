use sdql::rs;

#[test]
fn tpch_q1() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q1.sdql"));
    let _rs = rs!(src);
    // println!("{_rs}");
}

#[test]
fn tpch_q3() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q3.sdql"));
    let _rs = rs!(src);
    // println!("{_rs}");
}

#[test]
fn tpch_q6() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/progs/tpch/q6.sdql"));
    let _rs = rs!(src);
    // println!("{_rs}");
}
