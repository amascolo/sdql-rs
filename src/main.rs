#![feature(stmt_expr_attributes)]

use ordered_float::OrderedFloat;
use sdql::runtime::{Date, TRUE};
use sdql::runtime::{HashMap, Record};
use sdql::tpch::q3::TypeQ3;
use sdql::tpch::q6::TypeQ6;
use sdql::tpch::types::Lineitem;
use sdql::{date, load};

fn main() {
    println!("{:?}", q3()); // TODO display
    println!("{}", q6());
}

fn q6() -> TypeQ6 {
    let lineitem: Lineitem = load!(
        l_orderkey : i32, l_partkey : i32, l_suppkey : i32, l_linenumber : i32,
        l_quantity : OrderedFloat < f64 >, l_extendedprice : OrderedFloat < f64 >,
        l_discount : OrderedFloat < f64 >, l_tax : OrderedFloat < f64 >, l_returnflag :
        String, l_linestatus : String, l_shipdate : Date, l_commitdate : Date,
        l_receiptdate : Date, l_shipinstruct : String, l_shipmode : String, l_comment :
        String
    )("datasets/tpch_datasets/SF_0.01/lineitem.tbl")
    .unwrap();
    (0..lineitem.16)
        .filter(|&i| {
            OrderedFloat(0.05f64) <= lineitem.6[i]
                && lineitem.6[i] <= OrderedFloat(0.07f64)
                && lineitem.4[i] < OrderedFloat(24.0f64)
                && date!(19940101) <= lineitem.10[i]
                && lineitem.10[i] < date!(19950101)
        })
        .map(|i| lineitem.5[i] * lineitem.6[i])
        .sum()
}

fn q3() -> TypeQ3 {
    let customer = load!(
        c_custkey : i32, c_name : String, c_address : String, c_nationkey : i32, c_phone
        : String, c_acctbal : OrderedFloat < f64 >, c_mktsegment : String, c_comment :
        String
    )("datasets/tpch_datasets/SF_0.01/customer.tbl")
    .unwrap();
    let orders = load!(
        o_orderkey : i32, o_custkey : i32, o_orderstatus : String, o_totalprice :
        OrderedFloat < f64 >, o_orderdate : Date, o_orderpriority : String, o_clerk :
        String, o_shippriority : i32, o_comment : String
    )("datasets/tpch_datasets/SF_0.01/orders.tbl")
    .unwrap();
    let lineitem = load!(
        l_orderkey : i32, l_partkey : i32, l_suppkey : i32, l_linenumber : i32,
        l_quantity : OrderedFloat < f64 >, l_extendedprice : OrderedFloat < f64 >,
        l_discount : OrderedFloat < f64 >, l_tax : OrderedFloat < f64 >, l_returnflag :
        String, l_linestatus : String, l_shipdate : Date, l_commitdate : Date,
        l_receiptdate : Date, l_shipinstruct : String, l_shipmode : String, l_comment :
        String
    )("datasets/tpch_datasets/SF_0.01/lineitem.tbl")
    .unwrap();
    let c_h: HashMap<i32, Record<(i32,)>> = (0..customer.8)
        .filter(|&i| customer.6[i] == "BUILDING")
        .fold(HashMap::new(), |mut acc, i| {
            acc[&customer.0[i]] += Record::new((customer.0[i],));
            acc
        });
    let o_h: HashMap<i32, Record<(Date, i32)>> = (0..orders.9)
        .filter(|&i| orders.4[i] < date!(19950315) && c_h.contains_key(&orders.1[i]))
        .fold(HashMap::new(), |mut acc, i| {
            acc[&orders.0[i]] += Record::new((orders.4[i], orders.7[i]));
            acc
        });
    let l_h: HashMap<Record<(i32, Date, i32)>, Record<(OrderedFloat<f64>,)>> = (0..lineitem.16)
        .filter(|&i| date!(19950315) < lineitem.10[i] && o_h.contains_key(&lineitem.0[i]))
        .fold(HashMap::new(), |mut acc, i| {
            acc[&Record::new((lineitem.0[i], o_h[&lineitem.0[i]].0, o_h[&lineitem.0[i]].1))] +=
                Record::new((lineitem.5[i] * (OrderedFloat(1f64) - lineitem.6[i]),));
            acc
        });
    l_h.iter().fold(HashMap::new(), |mut acc: TypeQ3, (k, v)| {
        acc[&Record::new((k.0, k.1, k.2, v.0))] += TRUE;
        acc
    })
}
