#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_03(customer: &Customer, orders: &Orders, lineitem: &Lineitem) -> TypeQ3 {
    let mut c_h: HashMap<i32, Record<(i32,)>> = (0..customer.8)
        .into_iter()
        .filter(|&i| customer.6[i as usize] == VarChar::from_str("BUILDING").unwrap())
        .map(|i| {
            (
                customer.0[i as usize],
                Record::new((customer.0[i as usize],)),
            )
        })
        .collect();
    let mut o_h: HashMap<i32, Record<(Date, i32)>> = (0..orders.9)
        .into_iter()
        .filter(|&i| {
            orders.4[i as usize] < date!(19950315) && c_h.contains_key(&orders.1[i as usize])
        })
        .map(|i| {
            (
                orders.0[i as usize],
                Record::new((orders.4[i as usize], orders.7[i as usize])),
            )
        })
        .collect();
    let mut l_h: HashMap<Record<(i32, Date, i32)>, Record<(OrderedFloat<f64>,)>> = (0..lineitem.16)
        .into_iter()
        .filter(|&i| {
            date!(19950315) < lineitem.10[i as usize] && o_h.contains_key(&lineitem.0[i as usize])
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<Record<(i32, Date, i32)>, Record<(OrderedFloat<f64>,)>>, i| {
                acc[&Record::new((
                    lineitem.0[i as usize],
                    o_h[&lineitem.0[i as usize]].0,
                    o_h[&lineitem.0[i as usize]].1,
                ))] += Record::new((
                    lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]),
                ));
                acc
            },
        );
    l_h.into_iter()
        .map(|(k, v)| (Record::new((k.0, k.1, k.2, v.0)), TRUE))
        .collect()
}

pub fn tpch_03_parallel(customer: &Customer, orders: &Orders, lineitem: &Lineitem) -> TypeQ3 {
    let mut c_h: HashMap<i32, Record<(i32,)>> = (0..customer.8)
        .into_par_iter()
        .filter(|&i| customer.6[i as usize] == VarChar::from_str("BUILDING").unwrap())
        .map(|i| {
            (
                customer.0[i as usize],
                Record::new((customer.0[i as usize],)),
            )
        })
        .collect();
    let mut o_h: HashMap<i32, Record<(Date, i32)>> = (0..orders.9)
        .into_par_iter()
        .filter(|&i| {
            orders.4[i as usize] < date!(19950315) && c_h.contains_key(&orders.1[i as usize])
        })
        .map(|i| {
            (
                orders.0[i as usize],
                Record::new((orders.4[i as usize], orders.7[i as usize])),
            )
        })
        .collect();
    let mut l_h: HashMap<Record<(i32, Date, i32)>, Record<(OrderedFloat<f64>,)>> = (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| {
            date!(19950315) < lineitem.10[i as usize] && o_h.contains_key(&lineitem.0[i as usize])
        })
        .fold(
            HashMap::default,
            |mut acc: HashMap<Record<(i32, Date, i32)>, Record<(OrderedFloat<f64>,)>>, i| {
                acc[&Record::new((
                    lineitem.0[i as usize],
                    o_h[&lineitem.0[i as usize]].0,
                    o_h[&lineitem.0[i as usize]].1,
                ))] += Record::new((
                    lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]),
                ));
                acc
            },
        )
        .sum();
    l_h.into_par_iter()
        .map(|(k, v)| (Record::new((k.0, k.1, k.2, v.0)), TRUE))
        .collect()
}
