use sdql_runtime::{Date, Real, VarChar};

pub type Customer = (
    /* custkey */ Vec<i32>,
    /* name */ Vec<VarChar<25>>,
    /* address */ Vec<VarChar<40>>,
    /* nationkey */ Vec<i32>,
    /* phone */ Vec<VarChar<15>>,
    /* acctbal */ Vec<Real<f64>>,
    /* mktsegment */ Vec<VarChar<10>>,
    /* comment */ Vec<VarChar<117>>,
    /* size */ usize,
);

pub type Orders = (
    /* orderkey */ Vec<i32>,
    /* custkey */ Vec<i32>,
    /* orderstatus */ Vec<VarChar<1>>,
    /* totalprice */ Vec<Real<f64>>,
    /* orderdate */ Vec<Date>,
    /* orderpriority */ Vec<VarChar<15>>,
    /* clerk */ Vec<VarChar<15>>,
    /* shippriority */ Vec<i32>,
    /* comment */ Vec<VarChar<117>>,
    /* size */ usize,
);

pub type Lineitem = (
    /* orderkey */ Vec<i32>,
    /* partkey */ Vec<i32>,
    /* suppkey */ Vec<i32>,
    /* linenumber */ Vec<i32>,
    /* quantity */ Vec<Real<f64>>,
    /* extendedprice */ Vec<Real<f64>>,
    /* discount */ Vec<Real<f64>>,
    /* tax */ Vec<Real<f64>>,
    /* returnflag */ Vec<VarChar<1>>,
    /* linestatus */ Vec<VarChar<1>>,
    /* shipdate */ Vec<Date>,
    /* commitdate */ Vec<Date>,
    /* receiptdate */ Vec<Date>,
    /* shipinstruct */ Vec<VarChar<25>>,
    /* shipmode */ Vec<VarChar<10>>,
    /* comment */ Vec<VarChar<117>>,
    /* size */ usize,
);
