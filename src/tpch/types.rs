use sdql_runtime::{Date, HashSet, OrderedFloat, Record, VarChar};

pub type TypeQ1 = HashSet<
    Record<(
        VarChar<1>,
        VarChar<1>,
        OrderedFloat<f64>,
        OrderedFloat<f64>,
        OrderedFloat<f64>,
        OrderedFloat<f64>,
        i32,
    )>,
>;

pub type TypeQ2 = HashSet<
    Record<(
        OrderedFloat<f64>,
        VarChar<25>,
        VarChar<25>,
        i32,
        VarChar<25>,
        VarChar<15>,
        VarChar<40>,
        VarChar<101>,
    )>,
>;

pub type TypeQ3 = HashSet<Record<(i32, Date, i32, OrderedFloat<f64>)>>;

pub type TypeQ4 = HashSet<Record<(VarChar<25>, i32)>>;

pub type TypeQ5 = HashSet<Record<(VarChar<25>, OrderedFloat<f64>)>>;

pub type TypeQ6 = OrderedFloat<f64>;

pub type TypeQ7 = HashSet<Record<(VarChar<25>, VarChar<25>, i32, OrderedFloat<f64>)>>;

pub type TypeQ8 = HashSet<Record<(i32, i32)>>;

pub type TypeQ9 = HashSet<Record<(VarChar<25>, i32, OrderedFloat<f64>)>>;

pub type TypeQ10 = HashSet<
    Record<(
        i32,
        VarChar<25>,
        OrderedFloat<f64>,
        OrderedFloat<f64>,
        VarChar<25>,
        VarChar<15>,
        VarChar<40>,
        VarChar<117>,
    )>,
>;

pub type TypeQ18 = HashSet<
    Record<(
        VarChar<25>,
        i32,
        i32,
        Date,
        OrderedFloat<f64>,
        OrderedFloat<f64>,
    )>,
>;

pub type Customer = (
    /* custkey */ Vec<i32>,
    /* name */ Vec<VarChar<25>>,
    /* address */ Vec<VarChar<40>>,
    /* nationkey */ Vec<i32>,
    /* phone */ Vec<VarChar<15>>,
    /* acctbal */ Vec<OrderedFloat<f64>>,
    /* mktsegment */ Vec<VarChar<10>>,
    /* comment */ Vec<VarChar<117>>,
    /* size */ usize,
);

pub type Orders = (
    /* orderkey */ Vec<i32>,
    /* custkey */ Vec<i32>,
    /* orderstatus */ Vec<VarChar<1>>,
    /* totalprice */ Vec<OrderedFloat<f64>>,
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
    /* quantity */ Vec<OrderedFloat<f64>>,
    /* extendedprice */ Vec<OrderedFloat<f64>>,
    /* discount */ Vec<OrderedFloat<f64>>,
    /* tax */ Vec<OrderedFloat<f64>>,
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
