use sdql::tpch::q3::parallel::q3_rayon;
use sdql::tpch::q3::sequential::q3;
use sdql::tpch::q6::parallel::q6_rayon;
use sdql::tpch::q6::sequential::q6;
use sdql_macros::sdql_static;

#[test]
fn q3_works() {
    let actual = q3("0.01").unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/3.sdql"));
    // assert_eq!(actual, expected);
    assert_eq!(format!("{actual}"), format!("{expected}"));
}

#[test]
fn q3_rayon_works() {
    let actual = q3_rayon("0.01").unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/3.sdql"));
    // assert_eq!(actual, expected);
    assert_eq!(format!("{actual}"), format!("{expected}"));
}

#[test]
fn q6_works() {
    let actual = q6("0.01").unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/6.sdql"));
    // assert_eq!(actual, expected);
    assert_eq!(format!("{actual}"), format!("{expected}"));
}

#[test]
fn q6_rayon_works() {
    let actual = q6_rayon("0.01").unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/6.sdql"));
    // assert_eq!(actual, expected);
    assert_eq!(format!("{actual}"), format!("{expected}"));
}
