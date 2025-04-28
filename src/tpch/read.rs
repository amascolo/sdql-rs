use crate::tpch::types::*;
use sdql_runtime::{load, Date, OrderedFloat, VarChar};

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

pub fn read_part() -> fn(&str) -> Result<Part, Box<dyn std::error::Error>> {
    load!(
        p_partkey: i32,
        p_name: VarChar<55>,
        p_mfgr: VarChar<25>,
        p_brand: VarChar<10>,
        p_type: VarChar<25>,
        p_size: i32,
        p_container: VarChar<10>,
        p_retailprice: OrderedFloat<f64>,
        p_comment: VarChar<23>
    )
}

pub fn read_supplier() -> fn(&str) -> Result<Supplier, Box<dyn std::error::Error>> {
    load!(
        s_suppkey: i32,
        s_name: VarChar<25>,
        s_address: VarChar<40>,
        s_nationkey: i32,
        s_phone: VarChar<15>,
        s_acctbal: OrderedFloat<f64>,
        s_comment: VarChar<101>
    )
}

pub fn read_partsupp() -> fn(&str) -> Result<Partsupp, Box<dyn std::error::Error>> {
    load!(
        ps_partkey: i32,
        ps_suppkey: i32,
        ps_availqty: OrderedFloat <f64>,
        ps_supplycost: OrderedFloat <f64>,
        ps_comment: VarChar <199>
    )
}

pub fn read_nation() -> fn(&str) -> Result<Nation, Box<dyn std::error::Error>> {
    load!(
        n_nationkey: i32,
        n_name: VarChar<25>,
        n_regionkey: i32,
        n_comment: VarChar<152>
    )
}

pub fn read_region() -> fn(&str) -> Result<Region, Box<dyn std::error::Error>> {
    load!(
        r_regionkey: i32,
        r_name: VarChar<25>,
        r_comment:VarChar<152>
    )
}
