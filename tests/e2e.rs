use sdql::cli::run_tpch;

#[test]
fn tpch_q1() {
    let Ok(_actual) = run_tpch(1, "0.01") else {
        panic!()
    };
    // TODO
    // let expected = sdql_static!(include!("results/tpch/SF_0.01/3.sdql"));
    // let expected = include_str!("results/tpch/SF_0.01/1.sdql");
    // assert_eq!(actual, expected);
}

#[test]
fn tpch_q3() {
    assert!(run_tpch(3, "0.01").is_ok());
}

#[test]
fn tpch_q5() {
    assert!(run_tpch(5, "0.01").is_ok());
}

#[test]
fn tpch_q6() {
    assert!(run_tpch(6, "0.01").is_ok());
}

#[test]
fn tpch_q9() {
    assert!(run_tpch(9, "0.01").is_ok());
}

#[test]
fn tpch_q18() {
    assert!(run_tpch(18, "0.01").is_ok());
}
