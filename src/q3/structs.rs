#[allow(dead_code)]
pub struct Customer {
    pub custkey: Vec<i32>,
    pub name: Vec<String>,
    pub address: Vec<String>,
    pub nationkey: Vec<i32>,
    pub phone: Vec<String>,
    pub acctbal: Vec<f64>,
    pub mktsegment: Vec<String>,
    pub comment: Vec<String>,
    pub size: usize,
}

#[allow(dead_code)]
pub struct Orders {
    pub orderkey: Vec<i32>,
    pub custkey: Vec<i32>,
    pub orderstatus: Vec<String>,
    pub totalprice: Vec<f64>,
    pub orderdate: Vec<i32>,
    pub orderpriority: Vec<String>,
    pub clerk: Vec<String>,
    pub shippriority: Vec<i32>,
    pub comment: Vec<String>,
    pub size: usize,
}

#[allow(dead_code)]
pub struct Lineitem {
    pub orderkey: Vec<i32>,
    pub partkey: Vec<i32>,
    pub suppkey: Vec<i32>,
    pub linenumber: Vec<i32>,
    pub quantity: Vec<f64>,
    pub extendedprice: Vec<f64>,
    pub discount: Vec<f64>,
    pub tax: Vec<f64>,
    pub returnflag: Vec<String>,
    pub linestatus: Vec<String>,
    pub shipdate: Vec<i32>,
    pub commitdate: Vec<i32>,
    pub receiptdate: Vec<i32>,
    pub shipinstruct: Vec<String>,
    pub shipmode: Vec<String>,
    pub comment: Vec<String>,
    pub size: usize,
}
