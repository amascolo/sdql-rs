#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_18(customer: &Customer, orders: &Orders, lineitem: &Lineitem) -> TypeQ18 {
    let mut l_h: HashMap<i32, OrderedFloat<f64>> = (0..lineitem.16).into_iter().fold(
        HashMap::default(),
        |mut acc: HashMap<i32, OrderedFloat<f64>>, i| {
            acc[&lineitem.0[i as usize]] += lineitem.4[i as usize];
            acc
        },
    );
    let mut orderkeys: HashMap<i32, Bool> = l_h
        .into_iter()
        .filter(|&(l_orderkey, l_quantity)| OrderedFloat(300f64) < l_quantity)
        .map(|(l_orderkey, l_quantity)| (l_orderkey, TRUE))
        .collect();
    let mut custkey_to_name: HashMap<i32, Record<(VarChar<25>,)>> = (0..customer.8)
        .into_iter()
        .map(|i| {
            (
                customer.0[i as usize],
                Record::new((customer.1[i as usize],)),
            )
        })
        .collect();
    let mut o_h: HashMap<i32, Record<(VarChar<25>, i32, i32, Date, OrderedFloat<f64>)>> = (0
        ..orders.9)
        .into_iter()
        .filter(|&i| {
            orderkeys.contains_key(&orders.0[i as usize])
                && custkey_to_name.contains_key(&orders.1[i as usize])
        })
        .map(|i| {
            (
                orders.0[i as usize],
                Record::new((
                    custkey_to_name[&orders.1[i as usize]].0,
                    orders.1[i as usize],
                    orders.0[i as usize],
                    orders.4[i as usize],
                    orders.3[i as usize],
                )),
            )
        })
        .collect();
    let mut result_h: HashMap<
        Record<(VarChar<25>, i32, i32, Date, OrderedFloat<f64>)>,
        Record<(OrderedFloat<f64>,)>,
    > = (0..lineitem.16)
        .into_iter()
        .filter(|&i| o_h.contains_key(&lineitem.0[i as usize]))
        .fold(
            HashMap::default(),
            |mut acc: HashMap<
                Record<(VarChar<25>, i32, i32, Date, OrderedFloat<f64>)>,
                Record<(OrderedFloat<f64>,)>,
            >,
             i| {
                acc[&o_h[&lineitem.0[i as usize]]] += Record::new((lineitem.4[i as usize],));
                acc
            },
        );
    result_h
        .into_iter()
        .map(|(k, v)| (Record::new((k.0, k.1, k.2, k.3, k.4, v.0)), TRUE))
        .collect()
}

pub fn tpch_18_parallel(customer: &Customer, orders: &Orders, lineitem: &Lineitem) -> TypeQ18 {
    let mut l_h: HashMap<i32, OrderedFloat<f64>> = (0..lineitem.16)
        .into_par_iter()
        .fold(
            HashMap::default,
            |mut acc: HashMap<i32, OrderedFloat<f64>>, i| {
                acc[&lineitem.0[i as usize]] += lineitem.4[i as usize];
                acc
            },
        )
        .sum();
    let mut orderkeys: HashMap<i32, Bool> = l_h
        .into_par_iter()
        .filter(|&(l_orderkey, l_quantity)| OrderedFloat(300f64) < l_quantity)
        .map(|(l_orderkey, l_quantity)| (l_orderkey, TRUE))
        .collect();
    let mut custkey_to_name: HashMap<i32, Record<(VarChar<25>,)>> = (0..customer.8)
        .into_par_iter()
        .map(|i| {
            (
                customer.0[i as usize],
                Record::new((customer.1[i as usize],)),
            )
        })
        .collect();
    let mut o_h: HashMap<i32, Record<(VarChar<25>, i32, i32, Date, OrderedFloat<f64>)>> = (0
        ..orders.9)
        .into_par_iter()
        .filter(|&i| {
            orderkeys.contains_key(&orders.0[i as usize])
                && custkey_to_name.contains_key(&orders.1[i as usize])
        })
        .map(|i| {
            (
                orders.0[i as usize],
                Record::new((
                    custkey_to_name[&orders.1[i as usize]].0,
                    orders.1[i as usize],
                    orders.0[i as usize],
                    orders.4[i as usize],
                    orders.3[i as usize],
                )),
            )
        })
        .collect();
    let mut result_h: HashMap<
        Record<(VarChar<25>, i32, i32, Date, OrderedFloat<f64>)>,
        Record<(OrderedFloat<f64>,)>,
    > = (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| o_h.contains_key(&lineitem.0[i as usize]))
        .fold(
            HashMap::default,
            |mut acc: HashMap<
                Record<(VarChar<25>, i32, i32, Date, OrderedFloat<f64>)>,
                Record<(OrderedFloat<f64>,)>,
            >,
             i| {
                acc[&o_h[&lineitem.0[i as usize]]] += Record::new((lineitem.4[i as usize],));
                acc
            },
        )
        .sum();
    result_h
        .into_par_iter()
        .map(|(k, v)| (Record::new((k.0, k.1, k.2, k.3, k.4, v.0)), TRUE))
        .collect()
}
