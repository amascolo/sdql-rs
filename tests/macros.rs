use ordered_float::OrderedFloat;
use sdql_macros::{sdql_from_str, sdql_static};
use sdql_runtime::{date, HashMap, Record, FALSE, TRUE};

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

    let q1 = sdql_static!({
        <380456, 532348211.649998, 505822441.486102, 526165934.000839, 14876> -> true,
        <8971, 12384801.37, 11798257.208, 12282485.056933, 348> -> true,
        <742802, 1041502841.45, 989737518.634604, 1029418531.52335, 29181> -> true,
        <381449, 534594445.349999, 507996454.406699, 528524219.358906, 14902> -> true
    });
    println!("{q1}");

    let q1_from_str = sdql_from_str!(
        "
    {
        <380456, 532348211.649998, 505822441.486102, 526165934.000839, 14876> -> true,
        <8971, 12384801.37, 11798257.208, 12282485.056933, 348> -> true,
        <742802, 1041502841.45, 989737518.634604, 1029418531.52335, 29181> -> true,
        <381449, 534594445.349999, 507996454.406699, 528524219.358906, 14902> -> true
    }
    "
    );
    assert_eq!(q1_from_str, q1);

    let q1_from_src = sdql_static!(include!(
        "/Users/alex/repos/sdql-rs/tests/results/tpch/SF_0.01/q1_new.result"
    ));
    // // FIXME
    // let q1_from_src = sdql_static!(include!(concat!(
    //     env!("CARGO_MANIFEST_DIR"),
    //     "/tests/results/tpch/SF_0.01/q1_new.result"
    // )));
    assert_eq!(q1_from_src, q1);
}
