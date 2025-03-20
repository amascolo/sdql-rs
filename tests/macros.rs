use ordered_float::OrderedFloat;
use sdql_macros::sdql_static;
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
}
