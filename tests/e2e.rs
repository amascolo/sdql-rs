use approx::assert_abs_diff_eq;
use sdql::cli::run_tpch;
use sdql::tpch::types::{
    TypeQ1, TypeQ10, TypeQ11, TypeQ12, TypeQ13, TypeQ14, TypeQ15, TypeQ16, TypeQ17, TypeQ18, TypeQ19,
    TypeQ2, TypeQ20, TypeQ21, TypeQ22, TypeQ3, TypeQ4, TypeQ5, TypeQ6, TypeQ7, TypeQ8,
    TypeQ9,
};
use sdql_macros::sdql_static;
use sdql_runtime::{HashMap, OrderedFloat, Record, TRUE};

#[test]
fn tpch_1() {
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
fn tpch_2() {
    let buffer = run_tpch(2, "0.01").unwrap();
    let actual: TypeQ2 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/2.sdql"));
    assert_abs_diff_eq!(
        actual,
        expected,
        epsilon = ((1e-4, (), (), 0, (), (), (), ()), ())
    );
}

#[test]
fn tpch_3() {
    let buffer = run_tpch(3, "0.01").unwrap();
    let actual: TypeQ3 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/3.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = ((0, (), 0, 1e-4), ()));
}

#[test]
fn tpch_4() {
    let buffer = run_tpch(4, "0.01").unwrap();
    let actual: TypeQ4 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/4.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), 0), ()));
}

#[test]
fn tpch_5() {
    let buffer = run_tpch(5, "0.01").unwrap();
    let actual: TypeQ5 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/5.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), 1e-4), ()));
}

#[test]
fn tpch_6() {
    let buffer = run_tpch(6, "0.01").unwrap();
    let actual: TypeQ6 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/6.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
}

#[test]
fn tpch_7() {
    let buffer = run_tpch(7, "0.01").unwrap();
    let actual: TypeQ7 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/7.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), (), 0, 1e-4), ()));
}

#[test]
fn tpch_8() {
    let buffer = run_tpch(8, "0.01").unwrap();
    let actual: TypeQ8 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/8.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = ((0, 1e-4), ()));
}

#[test]
fn tpch_9() {
    let buffer = run_tpch(9, "0.01").unwrap();
    let actual: TypeQ9 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/9.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 1e-4), ()));
}

#[test]
fn tpch_10() {
    let buffer = run_tpch(10, "0.01").unwrap();
    let actual: TypeQ10 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/10.sdql"));
    assert_abs_diff_eq!(
        actual,
        expected,
        epsilon = ((0, (), 1e-4, 1e-4, (), (), (), ()), ())
    );
}

#[test]
fn tpch_11() {
    let buffer = run_tpch(11, "0.01").unwrap();
    let actual: TypeQ11 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/11.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = ((0, 1e-4), ()));
}

#[test]
fn tpch_12() {
    let buffer = run_tpch(12, "0.01").unwrap();
    let actual: TypeQ12 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/12.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 0), ()));
}

#[test]
fn tpch_13() {
    let buffer = run_tpch(13, "0.01").unwrap();
    let actual: TypeQ13 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/13.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = ((0, 0), ()));
}

#[test]
fn tpch_14() {
    let buffer = run_tpch(14, "0.01").unwrap();
    let actual: TypeQ14 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/14.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
}

#[test]
fn tpch_15() {
    let buffer = run_tpch(15, "0.01").unwrap();
    let actual: TypeQ15 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/15.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = ((0, (), (), (), 1e-4), ()));
}

#[test]
fn tpch_16() {
    let buffer = run_tpch(16, "0.01").unwrap();
    let actual: TypeQ16 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/16.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), (), 0, 0), ()));
}

#[test]
fn tpch_17() {
    let buffer = run_tpch(17, "0.01").unwrap();
    let actual: TypeQ17 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/17.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
}

#[test]
fn tpch_18() {
    let buffer = run_tpch(18, "0.01").unwrap();
    let actual: TypeQ18 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/18.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 0, (), 1e-4, 1e-4), ()));
}

#[test]
fn tpch_19() {
    let buffer = run_tpch(19, "0.01").unwrap();
    let actual: TypeQ19 = bincode::deserialize(&buffer).unwrap();
    // FIXME
    // let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/19.sdql"));
    let expected = HashMap::from([(Record::new((OrderedFloat(22923.028),)), TRUE)]);
    assert_abs_diff_eq!(actual, expected, epsilon = ((1e-4,), ()));
}

#[test]
fn tpch_20() {
    let buffer = run_tpch(20, "0.01").unwrap();
    let actual: TypeQ20 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/20.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), ()), ()));
}

#[test]
fn tpch_21() {
    let buffer = run_tpch(21, "0.01").unwrap();
    let actual: TypeQ21 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/21.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), 0), ()));
}

#[test]
fn tpch_22() {
    let buffer = run_tpch(22, "0.01").unwrap();
    let actual: TypeQ22 = bincode::deserialize(&buffer).unwrap();
    let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/22.sdql"));
    assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 1e-4), ()));
}

// TODO move SF1 behind a flag
//
// #[test]
// fn tpch_1_sf1() {
//     let buffer = run_tpch(1, "1").unwrap();
//     let actual: TypeQ1 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/1.sdql"));
//     assert_abs_diff_eq!(
//         actual,
//         expected,
//         // TODO change everywhere to 1e-3 (for SF=1 at least)
//         epsilon = (((), (), 1e-3, 1e-3, 1e-3, 1e-3, 0), ())
//     );
// }
//
// #[test]
// fn tpch_2_sf1() {
//     let buffer = run_tpch(2, "1").unwrap();
//     let actual: TypeQ2 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/2.sdql"));
//     assert_abs_diff_eq!(
//         actual,
//         expected,
//         epsilon = ((1e-4, (), (), 0, (), (), (), ()), ())
//     );
// }
//
// // FIXME sdql_static! blows up compilation time due to size of result
// #[test]
// fn tpch_3_sf1() {
//     let buffer = run_tpch(3, "1").unwrap();
//     let _actual: TypeQ3 = bincode::deserialize(&buffer).unwrap();
//     // let expected = sdql_static!(include!("tests/results/tpch/SF_1/3.sdql"));
//     // assert_abs_diff_eq!(actual, expected, epsilon = ((0, (), 0, 1e-4), ()));
// }
//
// #[test]
// fn tpch_4_sf1() {
//     let buffer = run_tpch(4, "1").unwrap();
//     let actual: TypeQ4 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/4.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0), ()));
// }
//
// #[test]
// fn tpch_5_sf1() {
//     let buffer = run_tpch(5, "1").unwrap();
//     let actual: TypeQ5 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/5.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = (((), 1e-4), ()));
// }
//
// #[test]
// fn tpch_6_sf1() {
//     let buffer = run_tpch(6, "1").unwrap();
//     let actual: TypeQ6 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/6.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
// }
//
// #[test]
// fn tpch_7_sf1() {
//     let buffer = run_tpch(7, "1").unwrap();
//     let actual: TypeQ7 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/7.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = (((), (), 0, 1e-4), ()));
// }
//
// #[test]
// fn tpch_8_sf1() {
//     let buffer = run_tpch(8, "1").unwrap();
//     let actual: TypeQ8 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/8.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = ((0, 1e-4), ()));
// }
//
// #[test]
// fn tpch_9_sf1() {
//     let buffer = run_tpch(9, "1").unwrap();
//     let actual: TypeQ9 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/9.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 1e-4), ()));
// }
//
// // FIXME sdql_static! blows up compilation time due to size of result
// #[test]
// fn tpch_10_sf1() {
//     let buffer = run_tpch(10, "1").unwrap();
//     let _actual: TypeQ10 = bincode::deserialize(&buffer).unwrap();
//     // TODO result file has last 2 columns merged and haven't rounded to 4dp the floats
//     // let expected = sdql_static!(include!("tests/results/tpch/SF_1/10.sdql"));
//     // assert_abs_diff_eq!(
//     //     actual,
//     //     expected,
//     //     epsilon = ((0, (), 1e-4, 1e-4, (), (), (), ()), ())
//     // );
// }
//
// #[test]
// fn tpch_11_sf1() {
//     let buffer = run_tpch(11, "1").unwrap();
//     let actual: TypeQ11 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/11.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = ((0, 1e-4), ()));
// }
//
// #[test]
// fn tpch_12_sf1() {
//     let buffer = run_tpch(12, "1").unwrap();
//     let actual: TypeQ12 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/12.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 0), ()));
// }
//
// #[test]
// fn tpch_13_sf1() {
//     let buffer = run_tpch(13, "1").unwrap();
//     let actual: TypeQ13 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/13.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = ((0, 0), ()));
// }
//
// #[test]
// fn tpch_14_sf1() {
//     let buffer = run_tpch(14, "1").unwrap();
//     let actual: TypeQ14 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/14.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
// }
//
// #[test]
// fn tpch_15_sf1() {
//     let buffer = run_tpch(15, "1").unwrap();
//     let actual: TypeQ15 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/15.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = ((0, (), (), (), 1e-4), ()));
// }
//
// // FIXME sdql_static! blows up compilation time due to size of result
// #[test]
// fn tpch_16_sf1() {
//     let buffer = run_tpch(16, "1").unwrap();
//     let _actual: TypeQ16 = bincode::deserialize(&buffer).unwrap();
//     // let expected = sdql_static!(include!("tests/results/tpch/SF_1/16.sdql"));
//     // assert_abs_diff_eq!(actual, expected, epsilon = (((), (), 0, 0), ()));
// }
//
// #[test]
// fn tpch_17_sf1() {
//     let buffer = run_tpch(17, "1").unwrap();
//     let actual: TypeQ17 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/17.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
// }
//
// #[test]
// fn tpch_18_sf1() {
//     let buffer = run_tpch(18, "1").unwrap();
//     let actual: TypeQ18 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/18.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 0, (), 1e-4, 1e-4), ()));
// }
//
// #[test]
// fn tpch_19_sf1() {
//     let buffer = run_tpch(19, "1").unwrap();
//     let actual: TypeQ19 = bincode::deserialize(&buffer).unwrap();
//     // FIXME
//     // let expected = sdql_static!(include!("tests/results/tpch/SF_1/19.sdql"));
//     let expected = HashMap::from([(Record::new((OrderedFloat(3083843.0578),)), TRUE)]);
//     assert_abs_diff_eq!(actual, expected, epsilon = ((1e-4,), ()));
// }
//
// #[test]
// fn tpch_20_sf1() {
//     let buffer = run_tpch(20, "1").unwrap();
//     let actual: TypeQ20 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/20.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = (((), ()), ()));
// }
//
// #[test]
// fn tpch_21_sf1() {
//     let buffer = run_tpch(21, "1").unwrap();
//     let actual: TypeQ21 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/21.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0), ()));
// }
//
// #[test]
// fn tpch_22_sf1() {
//     let buffer = run_tpch(22, "1").unwrap();
//     let actual: TypeQ22 = bincode::deserialize(&buffer).unwrap();
//     let expected = sdql_static!(include!("tests/results/tpch/SF_1/22.sdql"));
//     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 1e-4), ()));
// }
