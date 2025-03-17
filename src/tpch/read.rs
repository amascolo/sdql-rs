use crate::tpch::types::{Customer, Lineitem, Orders};
use ordered_float::OrderedFloat;
use sdql_runtime::{Date, VarChar, load};

pub fn read_customers() -> fn(&str) -> Result<Customer, Box<dyn std::error::Error>> {
    load!(
        custkey: i32,
        name: VarChar<25>,
        address: VarChar<40>,
        nationkey: i32,
        phone: VarChar<15>,
        acctbal: OrderedFloat<f64>,
        mktsegment: VarChar<10>,
        comment: VarChar<117>
    )
}

pub fn read_orders() -> fn(&str) -> Result<Orders, Box<dyn std::error::Error>> {
    load!(
        orderkey: i32,
        custkey: i32,
        orderstatus: VarChar<1>,
        totalprice: OrderedFloat<f64>,
        orderdate: Date,
        orderpriority: VarChar<15>,
        clerk: VarChar<15>,
        shippriority: i32,
        comment: VarChar<117>
    )
}

pub fn read_lineitems() -> fn(&str) -> Result<Lineitem, Box<dyn std::error::Error>> {
    load!(
        orderkey: i32,
        partkey: i32,
        suppkey: i32,
        linenumber: i32,
        quantity: OrderedFloat<f64>,
        extendedprice: OrderedFloat<f64>,
        discount: OrderedFloat<f64>,
        tax: OrderedFloat<f64>,
        returnflag: VarChar<1>,
        linestatus: VarChar<1>,
        shipdate: Date,
        commitdate: Date,
        receiptdate: Date,
        shipinstruct: VarChar<25>,
        shipmode: VarChar<10>,
        comment: VarChar<117>
    )
}
