#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn q10_query(
    customer: &Customer,
    orders: &Orders,
    lineitem: &Lineitem,
    nation: &Nation,
) -> TypeQ10 {
    let mut n_h: HashMap<i32, Record<(VarChar<25>,)>> = (0..nation.4)
        .into_iter()
        .map(|i| (nation.0[i as usize], Record::new((nation.1[i as usize],))))
        .collect();
    let mut c_h: HashMap<
        i32,
        Record<(
            i32,
            VarChar<25>,
            OrderedFloat<f64>,
            VarChar<40>,
            i32,
            VarChar<15>,
            VarChar<117>,
        )>,
    > = (0..customer.8)
        .into_iter()
        .map(|i| {
            (
                customer.0[i as usize],
                Record::new((
                    customer.0[i as usize],
                    customer.1[i as usize],
                    customer.5[i as usize],
                    customer.2[i as usize],
                    customer.3[i as usize],
                    customer.4[i as usize],
                    customer.7[i as usize],
                )),
            )
        })
        .collect();
    let mut o_h: HashMap<
        i32,
        Record<(
            i32,
            VarChar<25>,
            OrderedFloat<f64>,
            VarChar<40>,
            VarChar<15>,
            VarChar<117>,
            VarChar<25>,
        )>,
    > = (0..orders.9)
        .into_iter()
        .filter(|&i| {
            date!(19931001) <= orders.4[i as usize]
                && orders.4[i as usize] < date!(19940101)
                && c_h.contains_key(&orders.1[i as usize])
        })
        .map(|i| {
            (
                orders.0[i as usize],
                Record::new((
                    c_h[&orders.1[i as usize]].0,
                    c_h[&orders.1[i as usize]].1,
                    c_h[&orders.1[i as usize]].2,
                    c_h[&orders.1[i as usize]].3,
                    c_h[&orders.1[i as usize]].5,
                    c_h[&orders.1[i as usize]].6,
                    n_h[&c_h[&orders.1[i as usize]].4].0,
                )),
            )
        })
        .collect();
    let mut l_h: HashMap<
        Record<(
            i32,
            VarChar<25>,
            OrderedFloat<f64>,
            VarChar<25>,
            VarChar<40>,
            VarChar<15>,
            VarChar<117>,
        )>,
        OrderedFloat<f64>,
    > = (0..lineitem.16)
        .into_iter()
        .filter(|&i| {
            lineitem.8[i as usize] == VarChar::from_str("R").unwrap()
                && o_h.contains_key(&lineitem.0[i as usize])
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<
                Record<(
                    i32,
                    VarChar<25>,
                    OrderedFloat<f64>,
                    VarChar<25>,
                    VarChar<40>,
                    VarChar<15>,
                    VarChar<117>,
                )>,
                OrderedFloat<f64>,
            >,
             i| {
                acc[&Record::new((
                    o_h[&lineitem.0[i as usize]].0,
                    o_h[&lineitem.0[i as usize]].1,
                    o_h[&lineitem.0[i as usize]].2,
                    o_h[&lineitem.0[i as usize]].6,
                    o_h[&lineitem.0[i as usize]].3,
                    o_h[&lineitem.0[i as usize]].4,
                    o_h[&lineitem.0[i as usize]].5,
                ))] += lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]);
                acc
            },
        );
    l_h.into_iter()
        .map(|(k, v)| (Record::new((k.0, k.1, v, k.2, k.3, k.5, k.4, k.6)), TRUE))
        .collect()
}

pub fn q10_query_rayon(
    customer: &Customer,
    orders: &Orders,
    lineitem: &Lineitem,
    nation: &Nation,
) -> TypeQ10 {
    let mut n_h: HashMap<i32, Record<(VarChar<25>,)>> = (0..nation.4)
        .into_par_iter()
        .map(|i| (nation.0[i as usize], Record::new((nation.1[i as usize],))))
        .collect();
    let mut c_h: HashMap<
        i32,
        Record<(
            i32,
            VarChar<25>,
            OrderedFloat<f64>,
            VarChar<40>,
            i32,
            VarChar<15>,
            VarChar<117>,
        )>,
    > = (0..customer.8)
        .into_par_iter()
        .map(|i| {
            (
                customer.0[i as usize],
                Record::new((
                    customer.0[i as usize],
                    customer.1[i as usize],
                    customer.5[i as usize],
                    customer.2[i as usize],
                    customer.3[i as usize],
                    customer.4[i as usize],
                    customer.7[i as usize],
                )),
            )
        })
        .collect();
    let mut o_h: HashMap<
        i32,
        Record<(
            i32,
            VarChar<25>,
            OrderedFloat<f64>,
            VarChar<40>,
            VarChar<15>,
            VarChar<117>,
            VarChar<25>,
        )>,
    > = (0..orders.9)
        .into_par_iter()
        .filter(|&i| {
            date!(19931001) <= orders.4[i as usize]
                && orders.4[i as usize] < date!(19940101)
                && c_h.contains_key(&orders.1[i as usize])
        })
        .map(|i| {
            (
                orders.0[i as usize],
                Record::new((
                    c_h[&orders.1[i as usize]].0,
                    c_h[&orders.1[i as usize]].1,
                    c_h[&orders.1[i as usize]].2,
                    c_h[&orders.1[i as usize]].3,
                    c_h[&orders.1[i as usize]].5,
                    c_h[&orders.1[i as usize]].6,
                    n_h[&c_h[&orders.1[i as usize]].4].0,
                )),
            )
        })
        .collect();
    let mut l_h: HashMap<
        Record<(
            i32,
            VarChar<25>,
            OrderedFloat<f64>,
            VarChar<25>,
            VarChar<40>,
            VarChar<15>,
            VarChar<117>,
        )>,
        OrderedFloat<f64>,
    > = (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| {
            lineitem.8[i as usize] == VarChar::from_str("R").unwrap()
                && o_h.contains_key(&lineitem.0[i as usize])
        })
        .fold(
            HashMap::default,
            |mut acc: HashMap<
                Record<(
                    i32,
                    VarChar<25>,
                    OrderedFloat<f64>,
                    VarChar<25>,
                    VarChar<40>,
                    VarChar<15>,
                    VarChar<117>,
                )>,
                OrderedFloat<f64>,
            >,
             i| {
                acc[&Record::new((
                    o_h[&lineitem.0[i as usize]].0,
                    o_h[&lineitem.0[i as usize]].1,
                    o_h[&lineitem.0[i as usize]].2,
                    o_h[&lineitem.0[i as usize]].6,
                    o_h[&lineitem.0[i as usize]].3,
                    o_h[&lineitem.0[i as usize]].4,
                    o_h[&lineitem.0[i as usize]].5,
                ))] += lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]);
                acc
            },
        )
        .sum();
    l_h.into_par_iter()
        .map(|(k, v)| (Record::new((k.0, k.1, v, k.2, k.3, k.5, k.4, k.6)), TRUE))
        .collect()
}
