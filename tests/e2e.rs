use approx::assert_abs_diff_eq;
use sdql::cli::run_tpch;
use sdql::tpch::types::{TypeQ1, TypeQ18, TypeQ3, TypeQ5, TypeQ6, TypeQ9};
use sdql_macros::sdql_static;

#[test]
fn tpch_q1() {
    let buffer = run_tpch(1, "0.01").unwrap();
    let actual: TypeQ1 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/1.sdql"));
    assert_abs_diff_eq!(
        actual,
        expected,
        epsilon = (((), (), 1e-4, 1e-4, 1e-4, 1e-4, 0), ())
    );
}

#[test]
fn tpch_q3() {
    let buffer = run_tpch(3, "0.01").unwrap();
    let actual: TypeQ3 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/3.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = ((0, (), 0, 1e-4), ()));
}

#[test]
fn tpch_q5() {
    let buffer = run_tpch(5, "0.01").unwrap();
    let actual: TypeQ5 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/5.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), 1e-4), ()));
}

#[test]
fn tpch_q6() {
    let buffer = run_tpch(6, "0.01").unwrap();
    let actual: TypeQ6 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/6.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
}

#[test]
fn tpch_q9() {
    let buffer = run_tpch(9, "0.01").unwrap();
    let actual: TypeQ9 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/9.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 1e-4), ()));
}

#[test]
fn tpch_q18() {
    let buffer = run_tpch(18, "0.01").unwrap();
    let actual: TypeQ18 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/18.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 0, (), 1e-4, 1e-4), ()));
}
