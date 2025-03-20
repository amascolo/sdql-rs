use ordered_float::OrderedFloat;
use sdql_macros::sdql_static;
use sdql_runtime::{date, Bool, Date, HashMap, HashSet, Record, VarChar, FALSE, TRUE};

#[test]
fn sdql_static() {
    assert_eq!(sdql_static!(false), FALSE);
    assert_eq!(sdql_static!(true), TRUE);
    assert_eq!(sdql_static!(1), 1i32);
    assert_eq!(sdql_static!(-1), -1i32);
    assert_eq!(sdql_static!(3.14), OrderedFloat(3.14));
    assert_eq!(sdql_static!(date(20001231)), date!(20001231));
    assert_eq!(
        sdql_static!(<false, true, 1, -1, 3.14, date(20001231)>),
        Record::new((
            FALSE,
            TRUE,
            1i32,
            -1i32,
            OrderedFloat(3.14),
            date!(20001231),
        ))
    );
    assert_eq!(
        sdql_static!({<false, true, 1, -1, 3.14, date(20001231)> -> true}),
        HashMap::from([(
            sdql_static!(<false, true, 1, -1, 3.14, date(20001231)>),
            TRUE
        )])
    );

    let _: HashSet<
        Record<(
            VarChar<1>,
            VarChar<1>,
            i32,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            i32,
        )>,
    > = sdql_static!(include!("tests/results/tpch/SF_0.01/q1.result"));
    let _ = sdql_static!(include!("tests/results/tpch/SF_0.01/q3.result"));
    let _: HashMap<Record<(VarChar<10>, OrderedFloat<f64>)>, Bool> =
        sdql_static!(include!("tests/results/tpch/SF_0.01/q5.result"));
    let _ = sdql_static!(include!("tests/results/tpch/SF_0.01/q6.result"));
    let _: HashSet<Record<(VarChar<25>, i32, OrderedFloat<f64>)>> =
        sdql_static!(include!("tests/results/tpch/SF_0.01/q9.result"));
    let _: HashSet<Record<(VarChar<25>, i32, i32, Date, OrderedFloat<f64>, i32)>> =
        sdql_static!(include!("tests/results/tpch/SF_0.01/q18.result"));
}
