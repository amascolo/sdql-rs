// use sdql_runtime::*;
use sdql::tpch::q6::parallel::q6_rayon;
use sdql::tpch::q6::sequential::q6;
use sdql::utils::round;
use sdql_macros::sdql_static;

// FIXME rounding
// #[test]
// fn q3_works() {
//     let actual = q3("0.01").unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/q3.result"));
//     assert_eq!(actual, expected);
// }

// #[test]
// fn q3_rayon_works() {
//     let actual = q3_rayon("0.01").unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/q3.result"));
//     assert_eq!(actual, expected);
// }

#[test]
fn q6_works() {
    let actual = q6("0.01").unwrap();
    let actual = round(actual, 4);
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/q6.result"));
    assert_eq!(actual, expected);
}

#[test]
fn q6_rayon_works() {
    let actual = q6_rayon("0.01").unwrap();
    let actual = round(actual, 4);
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/q6.result"));
    assert_eq!(actual, expected);
}
