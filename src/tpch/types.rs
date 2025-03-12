use crate::runtime::Date;
use ordered_float::OrderedFloat;

pub type Customer = (
    /* custkey */ Vec<i32>,
    /* name */ Vec<String>,
    /* address */ Vec<String>,
    /* nationkey */ Vec<i32>,
    /* phone */ Vec<String>,
    /* acctbal */ Vec<OrderedFloat<f64>>,
    /* mktsegment */ Vec<String>,
    /* comment */ Vec<String>,
    /* size */ usize,
);

pub type Orders = (
    /* orderkey */ Vec<i32>,
    /* custkey */ Vec<i32>,
    /* orderstatus */ Vec<String>,
    /* totalprice */ Vec<OrderedFloat<f64>>,
    /* orderdate */ Vec<Date>,
    /* orderpriority */ Vec<String>,
    /* clerk */ Vec<String>,
    /* shippriority */ Vec<i32>,
    /* comment */ Vec<String>,
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
    /* returnflag */ Vec<String>,
    /* linestatus */ Vec<String>,
    /* shipdate */ Vec<Date>,
    /* commitdate */ Vec<Date>,
    /* receiptdate */ Vec<Date>,
    /* shipinstruct */ Vec<String>,
    /* shipmode */ Vec<String>,
    /* comment */ Vec<String>,
    /* size */ usize,
);
