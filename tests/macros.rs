use sdql::tpch::types::{TypeQ1, TypeQ18, TypeQ3, TypeQ5, TypeQ6, TypeQ9};
use sdql_macros::sdql_static;
use sdql_runtime::{date, HashMap, OrderedFloat, Record, FALSE, TRUE};

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
        HashMap::<_, _>::from_iter([(
            sdql_static!(<false, true, 1, -1, 3.14, date(20001231)>),
            TRUE
        )])
    );
    let _: TypeQ1 = sdql_static!(include!("tests/results/tpch/SF_0.01/1.sdql"));
    let _: TypeQ3 = sdql_static!(include!("tests/results/tpch/SF_0.01/3.sdql"));
    let _: TypeQ5 = sdql_static!(include!("tests/results/tpch/SF_0.01/5.sdql"));
    let _: TypeQ6 = sdql_static!(include!("tests/results/tpch/SF_0.01/6.sdql"));
    let _: TypeQ9 = sdql_static!(include!("tests/results/tpch/SF_0.01/9.sdql"));
    let _: TypeQ18 = sdql_static!(include!("tests/results/tpch/SF_0.01/18.sdql"));
}
