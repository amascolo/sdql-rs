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

pub type TypeQ4 = HashSet<Record<(VarChar<15>, i32)>>;

pub type TypeQ5 = HashSet<Record<(VarChar<25>, OrderedFloat<f64>)>>;

pub type TypeQ6 = OrderedFloat<f64>;

pub type TypeQ7 = HashSet<Record<(VarChar<25>, VarChar<25>, i32, OrderedFloat<f64>)>>;

pub type TypeQ8 = HashSet<Record<(i32, OrderedFloat<f64>)>>;

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

pub type TypeQ11 = HashSet<Record<(i32, OrderedFloat<f64>)>>;

pub type TypeQ12 = HashSet<Record<(VarChar<10>, i32, i32)>>;

pub type TypeQ13 = HashSet<Record<(i32, i32)>>;

pub type TypeQ14 = OrderedFloat<f64>;

pub type TypeQ15 = HashSet<
    Record<(
        i32,
        VarChar<25>,
        VarChar<40>,
        VarChar<15>,
        OrderedFloat<f64>,
    )>,
>;

pub type TypeQ16 = HashSet<Record<(VarChar<10>, VarChar<25>, i32, i32)>>;

pub type TypeQ17 = OrderedFloat<f64>;

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

pub type TypeQ19 = HashSet<Record<(OrderedFloat<f64>,)>>;

pub type TypeQ20 = HashSet<Record<(VarChar<25>, VarChar<40>)>>;

pub type TypeQ21 = HashSet<Record<(VarChar<25>, i32)>>;

pub type TypeQ22 = HashSet<Record<(VarChar<2>, i32, OrderedFloat<f64>)>>;

pub type Customer = (
    Vec<i32>,
    Vec<VarChar<25>>,
    Vec<VarChar<40>>,
    Vec<i32>,
    Vec<VarChar<15>>,
    Vec<OrderedFloat<f64>>,
    Vec<VarChar<10>>,
    Vec<VarChar<117>>,
    usize,
);

pub type Orders = (
    Vec<i32>,
    Vec<i32>,
    Vec<VarChar<1>>,
    Vec<OrderedFloat<f64>>,
    Vec<Date>,
    Vec<VarChar<15>>,
    Vec<VarChar<15>>,
    Vec<i32>,
    Vec<VarChar<117>>,
    usize,
);

pub type Lineitem = (
    Vec<i32>,
    Vec<i32>,
    Vec<i32>,
    Vec<i32>,
    Vec<OrderedFloat<f64>>,
    Vec<OrderedFloat<f64>>,
    Vec<OrderedFloat<f64>>,
    Vec<OrderedFloat<f64>>,
    Vec<VarChar<1>>,
    Vec<VarChar<1>>,
    Vec<Date>,
    Vec<Date>,
    Vec<Date>,
    Vec<VarChar<25>>,
    Vec<VarChar<10>>,
    Vec<VarChar<117>>,
    usize,
);

pub type Part = (
    Vec<i32>,
    Vec<VarChar<55>>,
    Vec<VarChar<25>>,
    Vec<VarChar<10>>,
    Vec<VarChar<25>>,
    Vec<i32>,
    Vec<VarChar<10>>,
    Vec<OrderedFloat<f64>>,
    Vec<VarChar<23>>,
    usize,
);

pub type Supplier = (
    Vec<i32>,
    Vec<VarChar<25>>,
    Vec<VarChar<40>>,
    Vec<i32>,
    Vec<VarChar<15>>,
    Vec<OrderedFloat<f64>>,
    Vec<VarChar<101>>,
    usize,
);

pub type Partsupp = (
    Vec<i32>,
    Vec<i32>,
    Vec<OrderedFloat<f64>>,
    Vec<OrderedFloat<f64>>,
    Vec<VarChar<199>>,
    usize,
);

pub type Nation = (
    Vec<i32>,
    Vec<VarChar<25>>,
    Vec<i32>,
    Vec<VarChar<152>>,
    usize,
);

pub type Region = (Vec<i32>, Vec<VarChar<25>>, Vec<VarChar<152>>, usize);
