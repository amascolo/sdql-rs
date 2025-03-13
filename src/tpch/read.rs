use crate::load;
use crate::runtime::Date;
use crate::tpch::types::{Customer, Lineitem, Orders};
use arrayvec::ArrayString;
use ordered_float::OrderedFloat;

pub fn read_customers() -> fn(&str) -> Result<Customer, Box<dyn std::error::Error>> {
    load!(
        custkey: i32,
        name: ArrayString<25>,
        address: ArrayString<40>,
        nationkey: i32,
        phone: ArrayString<15>,
        acctbal: OrderedFloat<f64>,
        mktsegment: ArrayString<10>,
        comment: ArrayString<117>
    )
}

pub fn read_orders() -> fn(&str) -> Result<Orders, Box<dyn std::error::Error>> {
    load!(
        orderkey: i32,
        custkey: i32,
        orderstatus: ArrayString<1>,
        totalprice: OrderedFloat<f64>,
        orderdate: Date,
        orderpriority: ArrayString<15>,
        clerk: ArrayString<15>,
        shippriority: i32,
        comment: ArrayString<117>
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
        returnflag: ArrayString<1>,
        linestatus: ArrayString<1>,
        shipdate: Date,
        commitdate: Date,
        receiptdate: Date,
        shipinstruct: ArrayString<25>,
        shipmode: ArrayString<10>,
        comment: ArrayString<117>
    )
}
