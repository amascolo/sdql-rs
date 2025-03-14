#![feature(stmt_expr_attributes)]
#![allow(dead_code, unused_variables)] // TODO remove

use ordered_float::OrderedFloat;
use sdql::runtime::Bool;
use sdql::runtime::{Date, HashMap, Record, VarChar, TRUE};
use sdql::tpch::q3::TypeQ3;
use sdql::tpch::q6::TypeQ6;
use sdql::{date, load};
use std::str::FromStr;

fn main() {
    println!("{}", q1());
    println!("{}", q3());
    println!("{}", q5());
    println!("{}", q6());
    // println!("{}", q9()); // FIXME vectors
    println!("{}", q18());
}

type TypeQ1 = HashMap<
    Record<(
        VarChar<1>,
        VarChar<1>,
        OrderedFloat<f64>,
        OrderedFloat<f64>,
        OrderedFloat<f64>,
        OrderedFloat<f64>,
        i32,
    )>,
    Bool,
>;

fn q1() -> TypeQ1 {
    let lineitem = load!(
        l_orderkey : i32, l_partkey : i32, l_suppkey : i32, l_linenumber : i32,
        l_quantity : OrderedFloat < f64 >, l_extendedprice : OrderedFloat < f64 >,
        l_discount : OrderedFloat < f64 >, l_tax : OrderedFloat < f64 >, l_returnflag :
        VarChar < 1 >, l_linestatus : VarChar < 1 >, l_shipdate : Date,
        l_commitdate : Date, l_receiptdate : Date, l_shipinstruct : VarChar < 25 >,
        l_shipmode : VarChar < 10 >, l_comment : VarChar < 44 >
    )("datasets/tpch_datasets/SF_0.01/lineitem.tbl")
    .unwrap();
    let l_h: HashMap<
        Record<(VarChar<1>, VarChar<1>)>,
        Record<(
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            i32,
        )>,
    > = (0..lineitem.16)
        .filter(|&i| lineitem.10[i] <= date!(19980902))
        .fold(HashMap::new(), |mut acc, i| {
            acc[&Record::new((lineitem.8[i], lineitem.9[i]))] += Record::new((
                lineitem.4[i],
                lineitem.5[i],
                lineitem.5[i] * (OrderedFloat(1f64) - lineitem.6[i]),
                lineitem.5[i]
                    * (OrderedFloat(1f64) - lineitem.6[i])
                    * (OrderedFloat(1f64) + lineitem.7[i]),
                1i32,
            ));
            acc
        });
    l_h.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc[&Record::new((k.0, k.1, v.0, v.1, v.2, v.3, v.4))] += TRUE;
        acc
    })
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

type TypeQ5 = HashMap<Record<(VarChar<25>, OrderedFloat<f64>)>, Bool>;

fn q5() -> TypeQ5 {
    let customer = load!(
        c_custkey : i32, c_name : VarChar < 25 >, c_address : VarChar < 40 >,
        c_nationkey : i32, c_phone : VarChar < 15 >, c_acctbal : OrderedFloat < f64
        >, c_mktsegment : VarChar < 10 >, c_comment : VarChar < 117 >
    )("datasets/tpch_datasets/SF_0.01/customer.tbl")
    .unwrap();
    let orders = load!(
        o_orderkey : i32, o_custkey : i32, o_orderstatus : VarChar < 1 >,
        o_totalprice : OrderedFloat < f64 >, o_orderdate : Date, o_orderpriority :
        VarChar < 15 >, o_clerk : VarChar < 15 >, o_shippriority : i32, o_comment
        : VarChar < 79 >
    )("datasets/tpch_datasets/SF_0.01/orders.tbl")
    .unwrap();
    let lineitem = load!(
        l_orderkey : i32, l_partkey : i32, l_suppkey : i32, l_linenumber : i32,
        l_quantity : OrderedFloat < f64 >, l_extendedprice : OrderedFloat < f64 >,
        l_discount : OrderedFloat < f64 >, l_tax : OrderedFloat < f64 >, l_returnflag :
        VarChar < 1 >, l_linestatus : VarChar < 1 >, l_shipdate : Date,
        l_commitdate : Date, l_receiptdate : Date, l_shipinstruct : VarChar < 25 >,
        l_shipmode : VarChar < 10 >, l_comment : VarChar < 44 >
    )("datasets/tpch_datasets/SF_0.01/lineitem.tbl")
    .unwrap();
    let supplier = load!(
        s_suppkey : i32, s_name : VarChar < 25 >, s_address : VarChar < 40 >,
        s_nationkey : i32, s_phone : VarChar < 15 >, s_acctbal : OrderedFloat < f64
        >, s_comment : VarChar < 101 >
    )("datasets/tpch_datasets/SF_0.01/supplier.tbl")
    .unwrap();
    let nation = load!(
        n_nationkey : i32, n_name : VarChar < 25 >, n_regionkey : i32, n_comment :
        VarChar < 152 >
    )("datasets/tpch_datasets/SF_0.01/nation.tbl")
    .unwrap();
    let region = load!(
        r_regionkey : i32, r_name : VarChar < 25 >, r_comment : VarChar < 152 >
    )("datasets/tpch_datasets/SF_0.01/region.tbl")
    .unwrap();
    let r_h: HashMap<i32, Record<(i32,)>> = (0..region.3)
        .filter(|&i| region.1[i] == VarChar::from_str("ASIA").unwrap())
        .fold(HashMap::new(), |mut acc, i| {
            acc[&region.0[i]] += Record::new((region.0[i],));
            acc
        });
    let n_h: HashMap<i32, VarChar<25>> = (0..nation.4)
        .filter(|&i| r_h.contains_key(&nation.2[i]))
        .fold(HashMap::new(), |mut acc, i| {
            acc[&nation.0[i]] += nation.1[i];
            acc
        });
    let c_h: HashMap<i32, Record<(VarChar<25>, i32)>> = (0..customer.8)
        .filter(|&i| n_h.contains_key(&customer.3[i]))
        .fold(HashMap::new(), |mut acc, i| {
            acc[&customer.0[i]] += Record::new((n_h[&customer.3[i]], customer.3[i]));
            acc
        });
    let o_h: HashMap<i32, Record<(VarChar<25>, i32)>> = (0..orders.9)
        .filter(|&i| {
            orders.4[i] < date!(19950101)
                && date!(19940101) <= orders.4[i]
                && c_h.contains_key(&orders.1[i])
        })
        .fold(HashMap::new(), |mut acc, i| {
            acc[&orders.0[i]] += Record::new((c_h[&orders.1[i]].0, c_h[&orders.1[i]].1));
            acc
        });
    let s_h: HashMap<Record<(i32, i32)>, i32> =
        (0..supplier.7).fold(HashMap::new(), |mut acc, i| {
            acc[&Record::new((supplier.0[i], supplier.3[i]))] += 1i32;
            acc
        });
    let l_h: HashMap<VarChar<25>, OrderedFloat<f64>> = (0..lineitem.16)
        .filter(|&i| {
            o_h.contains_key(&lineitem.0[i])
                && s_h.contains_key(&Record::new((lineitem.2[i], o_h[&lineitem.0[i]].1)))
        })
        .fold(HashMap::new(), |mut acc, i| {
            acc[&o_h[&lineitem.0[i]].0] += lineitem.5[i] * (OrderedFloat(1f64) - lineitem.6[i]);
            acc
        });
    l_h.iter().fold(HashMap::new(), |mut acc, (&k, &v)| {
        acc[&Record::new((k, v))] += TRUE;
        acc
    })
}

fn q6() -> TypeQ6 {
    let lineitem = load!(
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
                && lineitem.4[i] < OrderedFloat(24f64)
                && date!(19940101) <= lineitem.10[i]
                && lineitem.10[i] < date!(19950101)
        })
        .map(|i| lineitem.5[i] * lineitem.6[i])
        .sum()
}

type TypeQ9 = HashMap<Record<(VarChar<25>, Date, OrderedFloat<f64>)>, Bool>;

fn q9() -> TypeQ9 {
    let part = load!(
        p_partkey : i32, p_name : VarChar < 55 >, p_mfgr : VarChar < 25 >,
        p_brand : VarChar < 10 >, p_type : VarChar < 25 >, p_size : i32,
        p_container : VarChar < 10 >, p_retailprice : OrderedFloat < f64 >, p_comment
        : VarChar < 23 >
    )("datasets/tpch_datasets/SF_0.01/part.tbl")
    .unwrap();
    let supplier = load!(
        s_suppkey : i32, s_name : VarChar < 25 >, s_address : VarChar < 40 >,
        s_nationkey : i32, s_phone : VarChar < 15 >, s_acctbal : OrderedFloat < f64
        >, s_comment : VarChar < 101 >
    )("datasets/tpch_datasets/SF_0.01/supplier.tbl")
    .unwrap();
    let lineitem = load!(
        l_orderkey : i32, l_partkey : i32, l_suppkey : i32, l_linenumber : i32,
        l_quantity : OrderedFloat < f64 >, l_extendedprice : OrderedFloat < f64 >,
        l_discount : OrderedFloat < f64 >, l_tax : OrderedFloat < f64 >, l_returnflag :
        VarChar < 1 >, l_linestatus : VarChar < 1 >, l_shipdate : Date,
        l_commitdate : Date, l_receiptdate : Date, l_shipinstruct : VarChar < 25 >,
        l_shipmode : VarChar < 10 >, l_comment : VarChar < 44 >
    )("datasets/tpch_datasets/SF_0.01/lineitem.tbl")
    .unwrap();
    let partsupp = load!(
        ps_partkey : i32, ps_suppkey : i32, ps_availqty : OrderedFloat < f64 >,
        ps_supplycost : OrderedFloat < f64 >, ps_comment : VarChar < 199 >
    )("datasets/tpch_datasets/SF_0.01/partsupp.tbl")
    .unwrap();
    let orders = load!(
        o_orderkey : i32, o_custkey : i32, o_orderstatus : VarChar < 1 >,
        o_totalprice : OrderedFloat < f64 >, o_orderdate : Date, o_orderpriority :
        VarChar < 15 >, o_clerk : VarChar < 15 >, o_shippriority : i32, o_comment
        : VarChar < 79 >
    )("datasets/tpch_datasets/SF_0.01/orders.tbl")
    .unwrap();
    let nation = load!(
        n_nationkey : i32, n_name : VarChar < 25 >, n_regionkey : i32, n_comment :
        VarChar < 152 >
    )("datasets/tpch_datasets/SF_0.01/nation.tbl")
    .unwrap();
    let n_h: HashMap<i32, Record<(VarChar<25>,)>> =
        (0..nation.4).fold(HashMap::new(), |mut acc, i| {
            acc[&nation.0[i]] += Record::new((nation.1[i],));
            acc
        });
    let s_h = (0..supplier.7).fold(HashMap::new(), |mut acc, i| {
        acc[&supplier.0[i]] += n_h[&supplier.3[i]].0;
        acc
    });
    let p_h: HashMap<i32, Record<(i32,)>> = (0..part.9)
        .filter(|&i| part.1[i].contains(&"green"))
        .fold(HashMap::new(), |mut acc, i| {
            acc[&part.0[i]] += Record::new((part.0[i],));
            acc
        });
    let ps_h: HashMap<Record<(i32, i32)>, Record<(VarChar<25>, OrderedFloat<f64>)>> = (0..partsupp
        .5)
        .filter(|&i| p_h.contains_key(&partsupp.0[i]))
        .fold(HashMap::new(), |mut acc, i| {
            acc[&Record::new((partsupp.0[i], partsupp.1[i]))] +=
                Record::new((s_h[&partsupp.1[i]], partsupp.3[i]));
            acc
        });
    let o_h: Vec<Date> = (0..orders.9).fold(Vec::new(), |mut acc, i| {
        acc[usize::try_from(orders.0[i]).unwrap()] += orders.4[i];
        acc
    });
    let l_h: HashMap<Record<(VarChar<25>, Date)>, Record<(OrderedFloat<f64>,)>> = (0..lineitem.16)
        .filter(|&i| ps_h.contains_key(&Record::new((lineitem.1[i], lineitem.2[i]))))
        .fold(HashMap::new(), |mut acc, i| {
            acc[&Record::new((
                ps_h[&Record::new((lineitem.1[i], lineitem.2[i]))].0,
                o_h[usize::try_from(lineitem.0[i]).unwrap()],
            ))] += Record::new((lineitem.5[i] * (OrderedFloat(1f64) - lineitem.6[i])
                - ps_h[&Record::new((lineitem.1[i], lineitem.2[i]))].1 * lineitem.4[i],));
            acc
        });
    l_h.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc[&Record::new((k.0, k.1, v.0))] += TRUE;
        acc
    })
}

type TypeQ18 = HashMap<
    Record<(
        VarChar<25>,
        i32,
        i32,
        Date,
        OrderedFloat<f64>,
        OrderedFloat<f64>,
    )>,
    Bool,
>;

fn q18() -> TypeQ18 {
    let customer = load!(
        c_custkey : i32, c_name : VarChar < 25 >, c_address : VarChar < 40 >,
        c_nationkey : i32, c_phone : VarChar < 15 >, c_acctbal : OrderedFloat < f64
        >, c_mktsegment : VarChar < 10 >, c_comment : VarChar < 117 >
    )("datasets/tpch_datasets/SF_0.01/customer.tbl")
    .unwrap();
    let orders = load!(
        o_orderkey : i32, o_custkey : i32, o_orderstatus : VarChar < 1 >,
        o_totalprice : OrderedFloat < f64 >, o_orderdate : Date, o_orderpriority :
        VarChar < 15 >, o_clerk : VarChar < 15 >, o_shippriority : i32, o_comment
        : VarChar < 79 >
    )("datasets/tpch_datasets/SF_0.01/orders.tbl")
    .unwrap();
    let lineitem = load!(
        l_orderkey : i32, l_partkey : i32, l_suppkey : i32, l_linenumber : i32,
        l_quantity : OrderedFloat < f64 >, l_extendedprice : OrderedFloat < f64 >,
        l_discount : OrderedFloat < f64 >, l_tax : OrderedFloat < f64 >, l_returnflag :
        VarChar < 1 >, l_linestatus : VarChar < 1 >, l_shipdate : Date,
        l_commitdate : Date, l_receiptdate : Date, l_shipinstruct : VarChar < 25 >,
        l_shipmode : VarChar < 10 >, l_comment : VarChar < 44 >
    )("datasets/tpch_datasets/SF_0.01/lineitem.tbl")
    .unwrap();
    let l_h: HashMap<i32, OrderedFloat<f64>> =
        (0..lineitem.16).fold(HashMap::new(), |mut acc, i| {
            acc[&lineitem.0[i]] += lineitem.4[i];
            acc
        });
    let orderkeys: HashMap<i32, i32> = l_h
        .iter()
        .filter(|&(&l_orderkey, &l_quantity)| OrderedFloat(300f64) < l_quantity)
        .fold(HashMap::new(), |mut acc, (l_orderkey, l_quantity)| {
            acc[&l_orderkey] += 1i32;
            acc
        });
    let custkey_to_name: HashMap<i32, Record<(VarChar<25>,)>> =
        (0..customer.8).fold(HashMap::new(), |mut acc, i| {
            acc[&customer.0[i]] += Record::new((customer.1[i],));
            acc
        });
    let o_h = (0..orders.9)
        .filter(|&i| {
            orderkeys.contains_key(&orders.0[i]) && custkey_to_name.contains_key(&orders.1[i])
        })
        .fold(HashMap::new(), |mut acc, i| {
            acc[&orders.0[i]] += Record::new((
                custkey_to_name[&orders.1[i]].0,
                orders.1[i],
                orders.0[i],
                orders.4[i],
                orders.3[i],
            ));
            acc
        });
    let result_h: HashMap<
        Record<(VarChar<25>, i32, i32, Date, OrderedFloat<f64>)>,
        Record<(OrderedFloat<f64>,)>,
    > = (0..lineitem.16)
        .filter(|&i| o_h.contains_key(&lineitem.0[i]))
        .fold(HashMap::new(), |mut acc, i| {
            acc[&o_h[&lineitem.0[i]]] += Record::new((lineitem.4[i],));
            acc
        });
    result_h.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc[&Record::new((k.0, k.1, k.2, k.3, k.4, v.0))] += TRUE;
        acc
    })
}
