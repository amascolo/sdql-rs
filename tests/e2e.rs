use approx::assert_abs_diff_eq;
use sdql::cli::run_tpch;
use sdql::tpch::types::{
    TypeQ1, TypeQ10, TypeQ13, TypeQ14, TypeQ15, TypeQ16, TypeQ17, TypeQ18, TypeQ19, TypeQ2, TypeQ20,
    TypeQ21, TypeQ22, TypeQ3, TypeQ4, TypeQ5, TypeQ6, TypeQ7, TypeQ8, TypeQ9,
};
use sdql_macros::sdql_static;
use sdql_runtime::{HashMap, OrderedFloat, Record, TRUE};

mod sf_0_01 {
    use super::*;

    #[test]
    fn tpch_1() {
        let buffer = run_tpch::<false>(1, "0.01").unwrap();
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
        let buffer = run_tpch::<false>(2, "0.01").unwrap();
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
        let buffer = run_tpch::<false>(3, "0.01").unwrap();
        let actual: TypeQ3 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/3.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = ((0, (), 0, 1e-4), ()));
    }

    #[test]
    fn tpch_4() {
        let buffer = run_tpch::<false>(4, "0.01").unwrap();
        let actual: TypeQ4 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/4.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 0), ()));
    }

    #[test]
    fn tpch_5() {
        let buffer = run_tpch::<false>(5, "0.01").unwrap();
        let actual: TypeQ5 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/5.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 1e-4), ()));
    }

    #[test]
    fn tpch_6() {
        let buffer = run_tpch::<false>(6, "0.01").unwrap();
        let actual: TypeQ6 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/6.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
    }

    #[test]
    fn tpch_7() {
        let buffer = run_tpch::<false>(7, "0.01").unwrap();
        let actual: TypeQ7 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/7.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), (), 0, 1e-4), ()));
    }

    #[test]
    fn tpch_8() {
        let buffer = run_tpch::<false>(8, "0.01").unwrap();
        let actual: TypeQ8 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/8.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = ((0, 1e-4), ()));
    }

    #[test]
    fn tpch_9() {
        let buffer = run_tpch::<false>(9, "0.01").unwrap();
        let actual: TypeQ9 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/9.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 1e-4), ()));
    }

    #[test]
    fn tpch_10() {
        let buffer = run_tpch::<false>(10, "0.01").unwrap();
        let actual: TypeQ10 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/10.sdql"));
        assert_abs_diff_eq!(
            actual,
            expected,
            epsilon = ((0, (), 1e-4, 1e-4, (), (), (), ()), ())
        );
    }

    // FIXME
    //  let (ps_t_0, ps_t_1) = ps_t.decat();
    // #[test]
    // fn tpch_11() {
    //     let buffer = run_tpch::<false>(11, "0.01").unwrap();
    //     let actual: TypeQ11 = bincode::deserialize(&buffer).unwrap();
    //     let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/11.sdql"));
    //     assert_abs_diff_eq!(actual, expected, epsilon = ((0, 1e-4), ()));
    // }

    // FIXME
    //  .flat_map(|i| {
    //      l_h.remove(&orders.0[i as usize])
    //          .into_iter()
    //          .flat_map(move |inner_map| {
    //              inner_map
    //                  .into_iter()
    //                  .map(move |(l_shipmode, c)| (i, l_shipmode, c))
    //          })
    //  })
    // #[test]
    // fn tpch_12() {
    //     let buffer = run_tpch::<false>(12, "0.01").unwrap();
    //     let actual: TypeQ12 = bincode::deserialize(&buffer).unwrap();
    //     let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/12.sdql"));
    //     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 0), ()));
    // }

    #[test]
    fn tpch_13() {
        let buffer = run_tpch::<false>(13, "0.01").unwrap();
        let actual: TypeQ13 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/13.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = ((0, 0), ()));
    }

    #[test]
    fn tpch_14() {
        let buffer = run_tpch::<false>(14, "0.01").unwrap();
        let actual: TypeQ14 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/14.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
    }

    #[test]
    fn tpch_15() {
        let buffer = run_tpch::<false>(15, "0.01").unwrap();
        let actual: TypeQ15 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/15.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = ((0, (), (), (), 1e-4), ()));
    }

    #[test]
    fn tpch_16() {
        let buffer = run_tpch::<false>(16, "0.01").unwrap();
        let actual: TypeQ16 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/16.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), (), 0, 0), ()));
    }

    #[test]
    fn tpch_17() {
        let buffer = run_tpch::<false>(17, "0.01").unwrap();
        let actual: TypeQ17 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/17.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
    }

    #[test]
    fn tpch_18() {
        let buffer = run_tpch::<false>(18, "0.01").unwrap();
        let actual: TypeQ18 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/18.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 0, (), 1e-4, 1e-4), ()));
    }

    #[test]
    fn tpch_19() {
        let buffer = run_tpch::<false>(19, "0.01").unwrap();
        let actual: TypeQ19 = bincode::deserialize(&buffer).unwrap();
        // FIXME
        // let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/19.sdql"));
        let expected = HashMap::from([(Record::new((OrderedFloat(22923.028),)), TRUE)]);
        assert_abs_diff_eq!(actual, expected, epsilon = ((1e-4,), ()));
    }

    #[test]
    fn tpch_20() {
        let buffer = run_tpch::<false>(20, "0.01").unwrap();
        let actual: TypeQ20 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/20.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), ()), ()));
    }

    #[test]
    fn tpch_21() {
        let buffer = run_tpch::<false>(21, "0.01").unwrap();
        let actual: TypeQ21 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/21.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 0), ()));
    }

    #[test]
    fn tpch_22() {
        let buffer = run_tpch::<false>(22, "0.01").unwrap();
        let actual: TypeQ22 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_0.01/22.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 1e-4), ()));
    }
}

mod sf_1 {
    use super::*;

    #[test]
    fn tpch_1() {
        let buffer = run_tpch::<false>(1, "1").unwrap();
        let actual: TypeQ1 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/1.sdql"));
        assert_abs_diff_eq!(
            actual,
            expected,
            epsilon = (((), (), 1e-3, 1e-3, 1e-3, 1e-3, 0), ()) // note: was lowered to 1e-3
        );
    }

    #[test]
    fn tpch_2() {
        let buffer = run_tpch::<false>(2, "1").unwrap();
        let actual: TypeQ2 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/2.sdql"));
        assert_abs_diff_eq!(
            actual,
            expected,
            epsilon = ((1e-4, (), (), 0, (), (), (), ()), ())
        );
    }

    #[test]
    fn tpch_3() {
        let buffer = run_tpch::<false>(3, "1").unwrap();
        let _actual: TypeQ3 = bincode::deserialize(&buffer).unwrap();
        // FIXME sdql_static! blows up compilation time due to size of result
        // let expected = sdql_static!(include!("tests/results/tpch/SF_1/3.sdql"));
        // assert_abs_diff_eq!(actual, expected, epsilon = ((0, (), 0, 1e-4), ()));
    }

    #[test]
    fn tpch_4() {
        let buffer = run_tpch::<false>(4, "1").unwrap();
        let actual: TypeQ4 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/4.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 0), ()));
    }

    #[test]
    fn tpch_5() {
        let buffer = run_tpch::<false>(5, "1").unwrap();
        let actual: TypeQ5 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/5.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 1e-4), ()));
    }

    #[test]
    fn tpch_6() {
        let buffer = run_tpch::<false>(6, "1").unwrap();
        let actual: TypeQ6 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/6.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
    }

    #[test]
    fn tpch_7() {
        let buffer = run_tpch::<false>(7, "1").unwrap();
        let actual: TypeQ7 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/7.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), (), 0, 1e-4), ()));
    }

    #[test]
    fn tpch_8() {
        let buffer = run_tpch::<false>(8, "1").unwrap();
        let actual: TypeQ8 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/8.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = ((0, 1e-4), ()));
    }

    #[test]
    fn tpch_9() {
        let buffer = run_tpch::<false>(9, "1").unwrap();
        let actual: TypeQ9 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/9.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 1e-4), ()));
    }

    #[test]
    fn tpch_10() {
        let buffer = run_tpch::<false>(10, "1").unwrap();
        let _actual: TypeQ10 = bincode::deserialize(&buffer).unwrap();
        // FIXME sdql_static! blows up compilation time due to size of result
        // TODO result file has last 2 columns merged and there's no rounding to 4dp in the floats
        // let expected = sdql_static!(include!("tests/results/tpch/SF_1/10.sdql"));
        // assert_abs_diff_eq!(
        //     actual,
        //     expected,
        //     epsilon = ((0, (), 1e-4, 1e-4, (), (), (), ()), ())
        // );
    }

    // FIXME
    // #[test]
    // fn tpch_11() {
    //     let buffer = run_tpch::<false>(11, "1").unwrap();
    //     let actual: TypeQ11 = bincode::deserialize(&buffer).unwrap();
    //     let expected = sdql_static!(include!("tests/results/tpch/SF_1/11.sdql"));
    //     assert_abs_diff_eq!(actual, expected, epsilon = ((0, 1e-4), ()));
    // }

    // FIXME
    // #[test]
    // fn tpch_12() {
    //     let buffer = run_tpch::<false>(12, "1").unwrap();
    //     let actual: TypeQ12 = bincode::deserialize(&buffer).unwrap();
    //     let expected = sdql_static!(include!("tests/results/tpch/SF_1/12.sdql"));
    //     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 0), ()));
    // }

    #[test]
    fn tpch_13() {
        let buffer = run_tpch::<false>(13, "1").unwrap();
        let actual: TypeQ13 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/13.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = ((0, 0), ()));
    }

    #[test]
    fn tpch_14() {
        let buffer = run_tpch::<false>(14, "1").unwrap();
        let actual: TypeQ14 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/14.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
    }

    #[test]
    fn tpch_15() {
        let buffer = run_tpch::<false>(15, "1").unwrap();
        let actual: TypeQ15 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/15.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = ((0, (), (), (), 1e-4), ()));
    }

    #[test]
    fn tpch_16() {
        let buffer = run_tpch::<false>(16, "1").unwrap();
        let _actual: TypeQ16 = bincode::deserialize(&buffer).unwrap();
        // FIXME sdql_static! blows up compilation time due to size of result
        // let expected = sdql_static!(include!("tests/results/tpch/SF_1/16.sdql"));
        // assert_abs_diff_eq!(actual, expected, epsilon = (((), (), 0, 0), ()));
    }

    #[test]
    fn tpch_17() {
        let buffer = run_tpch::<false>(17, "1").unwrap();
        let actual: TypeQ17 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/17.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
    }

    #[test]
    fn tpch_18() {
        let buffer = run_tpch::<false>(18, "1").unwrap();
        let actual: TypeQ18 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/18.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 0, (), 1e-4, 1e-4), ()));
    }

    #[test]
    fn tpch_19() {
        let buffer = run_tpch::<false>(19, "1").unwrap();
        let actual: TypeQ19 = bincode::deserialize(&buffer).unwrap();
        // FIXME
        // let expected = sdql_static!(include!("tests/results/tpch/SF_1/19.sdql"));
        let expected = HashMap::from([(Record::new((OrderedFloat(3083843.0578),)), TRUE)]);
        assert_abs_diff_eq!(actual, expected, epsilon = ((1e-4,), ()));
    }

    #[test]
    fn tpch_20() {
        let buffer = run_tpch::<false>(20, "1").unwrap();
        let actual: TypeQ20 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/20.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), ()), ()));
    }

    #[test]
    fn tpch_21() {
        let buffer = run_tpch::<false>(21, "1").unwrap();
        let actual: TypeQ21 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/21.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 0), ()));
    }

    #[test]
    fn tpch_22() {
        let buffer = run_tpch::<false>(22, "1").unwrap();
        let actual: TypeQ22 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/22.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 1e-4), ()));
    }
}

// FIXME requires fixing hardcoded hack in codegen::gen_args
mod parallel {
    use super::*;

    #[test]
    fn tpch_1() {
        let buffer = run_tpch::<true>(1, "1").unwrap();
        let actual: TypeQ1 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/1.sdql"));
        assert_abs_diff_eq!(
            actual,
            expected,
            epsilon = (((), (), 1e-2, 1e-2, 1e-2, 1e-2, 0), ()) // note: was lowered to 1e-2
        );
    }

    #[test]
    fn tpch_2() {
        let buffer = run_tpch::<true>(2, "1").unwrap();
        let actual: TypeQ2 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/2.sdql"));
        assert_abs_diff_eq!(
            actual,
            expected,
            epsilon = ((1e-4, (), (), 0, (), (), (), ()), ())
        );
    }

    #[test]
    fn tpch_3() {
        let buffer = run_tpch::<true>(3, "1").unwrap();
        let _actual: TypeQ3 = bincode::deserialize(&buffer).unwrap();
        // FIXME sdql_static! blows up compilation time due to size of result
        // let expected = sdql_static!(include!("tests/results/tpch/SF_1/3.sdql"));
        // assert_abs_diff_eq!(actual, expected, epsilon = ((0, (), 0, 1e-4), ()));
    }

    // FIXME
    //  .sum();
    //   ^^^ value of type `Vec<i32>` cannot be made by summing a `std::iter::Iterator<Item=Vec<i32>>`
    // #[test]
    // fn tpch_4() {
    //     let buffer = run_tpch::<true>(4, "1").unwrap();
    //     let actual: TypeQ4 = bincode::deserialize(&buffer).unwrap();
    //     let expected = sdql_static!(include!("tests/results/tpch/SF_1/4.sdql"));
    //     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0), ()));
    // }

    #[test]
    fn tpch_5() {
        let buffer = run_tpch::<true>(5, "1").unwrap();
        let actual: TypeQ5 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/5.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 1e-4), ()));
    }

    #[test]
    fn tpch_6() {
        let buffer = run_tpch::<true>(6, "1").unwrap();
        let actual: TypeQ6 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/6.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
    }

    #[test]
    fn tpch_7() {
        let buffer = run_tpch::<true>(7, "1").unwrap();
        let actual: TypeQ7 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/7.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), (), 0, 1e-4), ()));
    }

    // FIXME
    //  .sum();
    //   ^^^ value of type `Vec<i32>` cannot be made by summing a `std::iter::Iterator<Item=Vec<i32>>`
    // #[test]
    // fn tpch_8() {
    //     let buffer = run_tpch::<true>(8, "1").unwrap();
    //     let actual: TypeQ8 = bincode::deserialize(&buffer).unwrap();
    //     let expected = sdql_static!(include!("tests/results/tpch/SF_1/8.sdql"));
    //     assert_abs_diff_eq!(actual, expected, epsilon = ((0, 1e-4), ()));
    // }

    // FIXME
    //  .sum();
    //   ^^^ value of type `Vec<i32>` cannot be made by summing a `std::iter::Iterator<Item=Vec<i32>>`
    // #[test]
    // fn tpch_9() {
    //     let buffer = run_tpch::<true>(9, "1").unwrap();
    //     let actual: TypeQ9 = bincode::deserialize(&buffer).unwrap();
    //     let expected = sdql_static!(include!("tests/results/tpch/SF_1/9.sdql"));
    //     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 1e-4), ()));
    // }

    #[test]
    fn tpch_10() {
        let buffer = run_tpch::<true>(10, "1").unwrap();
        let _actual: TypeQ10 = bincode::deserialize(&buffer).unwrap();
        // FIXME sdql_static! blows up compilation time due to size of result
        // TODO result file has last 2 columns merged and there's no rounding to 4dp in the floats
        // let expected = sdql_static!(include!("tests/results/tpch/SF_1/10.sdql"));
        // assert_abs_diff_eq!(
        //     actual,
        //     expected,
        //     epsilon = ((0, (), 1e-4, 1e-4, (), (), (), ()), ())
        // );
    }

    // FIXME
    // #[test]
    // fn tpch_11() {
    //     let buffer = run_tpch::<true>(11, "1").unwrap();
    //     let actual: TypeQ11 = bincode::deserialize(&buffer).unwrap();
    //     let expected = sdql_static!(include!("tests/results/tpch/SF_1/11.sdql"));
    //     assert_abs_diff_eq!(actual, expected, epsilon = ((0, 1e-4), ()));
    // }

    // FIXME
    // #[test]
    // fn tpch_12() {
    //     let buffer = run_tpch::<true>(12, "1").unwrap();
    //     let actual: TypeQ12 = bincode::deserialize(&buffer).unwrap();
    //     let expected = sdql_static!(include!("tests/results/tpch/SF_1/12.sdql"));
    //     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 0), ()));
    // }

    #[test]
    fn tpch_13() {
        let buffer = run_tpch::<true>(13, "1").unwrap();
        let actual: TypeQ13 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/13.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = ((0, 0), ()));
    }

    #[test]
    fn tpch_14() {
        let buffer = run_tpch::<true>(14, "1").unwrap();
        let actual: TypeQ14 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/14.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
    }

    #[test]
    fn tpch_15() {
        let buffer = run_tpch::<true>(15, "1").unwrap();
        let actual: TypeQ15 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/15.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = ((0, (), (), (), 1e-4), ()));
    }

    // FIXME investigate several issues
    // #[test]
    // fn tpch_16() {
    //     let buffer = run_tpch::<true>(16, "1").unwrap();
    //     let _actual: TypeQ16 = bincode::deserialize(&buffer).unwrap();
    //     // FIXME sdql_static! blows up compilation time due to size of result
    //     // let expected = sdql_static!(include!("tests/results/tpch/SF_1/16.sdql"));
    //     // assert_abs_diff_eq!(actual, expected, epsilon = (((), (), 0, 0), ()));
    // }

    #[test]
    fn tpch_17() {
        let buffer = run_tpch::<true>(17, "1").unwrap();
        let actual: TypeQ17 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/17.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-4);
    }

    #[test]
    fn tpch_18() {
        let buffer = run_tpch::<true>(18, "1").unwrap();
        let actual: TypeQ18 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/18.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 0, (), 1e-4, 1e-4), ()));
    }

    #[test]
    fn tpch_19() {
        let buffer = run_tpch::<true>(19, "1").unwrap();
        let actual: TypeQ19 = bincode::deserialize(&buffer).unwrap();
        // FIXME
        // let expected = sdql_static!(include!("tests/results/tpch/SF_1/19.sdql"));
        let expected = HashMap::from([(Record::new((OrderedFloat(3083843.0578),)), TRUE)]);
        assert_abs_diff_eq!(actual, expected, epsilon = ((1e-4,), ()));
    }

    #[test]
    fn tpch_20() {
        let buffer = run_tpch::<true>(20, "1").unwrap();
        let actual: TypeQ20 = bincode::deserialize(&buffer).unwrap();
        let expected = sdql_static!(include!("tests/results/tpch/SF_1/20.sdql"));
        assert_abs_diff_eq!(actual, expected, epsilon = (((), ()), ()));
    }

    // FIXME
    //  .sum();
    //   ^^^ value of type `Vec<i32>` cannot be made by summing a `std::iter::Iterator<Item=Vec<i32>>`
    // #[test]
    // fn tpch_21() {
    //     let buffer = run_tpch::<true>(21, "1").unwrap();
    //     let actual: TypeQ21 = bincode::deserialize(&buffer).unwrap();
    //     let expected = sdql_static!(include!("tests/results/tpch/SF_1/21.sdql"));
    //     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0), ()));
    // }

    // FIXME
    //  .sum();
    //   ^^^ value of type `Vec<i32>` cannot be made by summing a `std::iter::Iterator<Item=Vec<i32>>`
    // #[test]
    // fn tpch_22() {
    //     let buffer = run_tpch::<true>(22, "1").unwrap();
    //     let actual: TypeQ22 = bincode::deserialize(&buffer).unwrap();
    //     let expected = sdql_static!(include!("tests/results/tpch/SF_1/22.sdql"));
    //     assert_abs_diff_eq!(actual, expected, epsilon = (((), 0, 1e-4), ()));
    // }
}
