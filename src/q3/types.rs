pub type Customer = (
    /* custkey */ Vec<i32>,
    /* name */ Vec<String>,
    /* address */ Vec<String>,
    /* nationkey */ Vec<i32>,
    /* phone */ Vec<String>,
    /* acctbal */ Vec<f64>,
    /* mktsegment */ Vec<String>,
    /* comment */ Vec<String>,
    /* size */ usize,
);

pub type Orders = (
    /* orderkey */ Vec<i32>,
    /* custkey */ Vec<i32>,
    /* orderstatus */ Vec<String>,
    /* totalprice */ Vec<f64>,
    /* orderdate */ Vec<i32>,
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
    /* quantity */ Vec<f64>,
    /* extendedprice */ Vec<f64>,
    /* discount */ Vec<f64>,
    /* tax */ Vec<f64>,
    /* returnflag */ Vec<String>,
    /* linestatus */ Vec<String>,
    /* shipdate */ Vec<i32>,
    /* commitdate */ Vec<i32>,
    /* receiptdate */ Vec<i32>,
    /* shipinstruct */ Vec<String>,
    /* shipmode */ Vec<String>,
    /* comment */ Vec<String>,
    /* size */ usize,
);
