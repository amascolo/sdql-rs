use crate::runtime::Date;
use arrayvec::ArrayString;
use ordered_float::OrderedFloat;

pub type Customer = (
    /* custkey */ Vec<i32>,
    /* name */ Vec<ArrayString<25>>,
    /* address */ Vec<ArrayString<40>>,
    /* nationkey */ Vec<i32>,
    /* phone */ Vec<ArrayString<15>>,
    /* acctbal */ Vec<OrderedFloat<f64>>,
    /* mktsegment */ Vec<ArrayString<10>>,
    /* comment */ Vec<ArrayString<117>>,
    /* size */ usize,
);

pub type Orders = (
    /* orderkey */ Vec<i32>,
    /* custkey */ Vec<i32>,
    /* orderstatus */ Vec<ArrayString<1>>,
    /* totalprice */ Vec<OrderedFloat<f64>>,
    /* orderdate */ Vec<Date>,
    /* orderpriority */ Vec<ArrayString<15>>,
    /* clerk */ Vec<ArrayString<15>>,
    /* shippriority */ Vec<i32>,
    /* comment */ Vec<ArrayString<117>>,
    /* size */ usize,
);

pub type Lineitem = (
    /* orderkey */ Vec<i32>,
    /* partkey */ Vec<i32>,
    /* suppkey */ Vec<i32>,
    /* linenumber */ Vec<i32>,
    /* quantity */ Vec<OrderedFloat<f64>>,
    /* extendedprice */ Vec<OrderedFloat<f64>>,
    /* discount */ Vec<OrderedFloat<f64>>,
    /* tax */ Vec<OrderedFloat<f64>>,
    /* returnflag */ Vec<ArrayString<1>>,
    /* linestatus */ Vec<ArrayString<1>>,
    /* shipdate */ Vec<Date>,
    /* commitdate */ Vec<Date>,
    /* receiptdate */ Vec<Date>,
    /* shipinstruct */ Vec<ArrayString<25>>,
    /* shipmode */ Vec<ArrayString<10>>,
    /* comment */ Vec<ArrayString<117>>,
    /* size */ usize,
);
