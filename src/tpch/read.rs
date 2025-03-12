use crate::load;
use crate::runtime::Date;
use crate::tpch::types::{Customer, Lineitem, Orders};
use ordered_float::OrderedFloat;

pub fn read_customers() -> fn(&str) -> Result<Customer, Box<dyn std::error::Error>> {
    load!(
        custkey: i32,
        name: String,
        address: String,
        nationkey: i32,
        phone: String,
        acctbal: OrderedFloat<f64>,
        mktsegment: String,
        comment: String
    )
}

pub fn read_orders() -> fn(&str) -> Result<Orders, Box<dyn std::error::Error>> {
    load!(
        orderkey: i32,
        custkey: i32,
        orderstatus: String,
        totalprice: OrderedFloat<f64>,
        orderdate: Date,
        orderpriority: String,
        clerk: String,
        shippriority: i32,
        comment: String
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
        returnflag: String,
        linestatus: String,
        shipdate: Date,
        commitdate: Date,
        receiptdate: Date,
        shipinstruct: String,
        shipmode: String,
        comment: String
    )
}
