#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_13(orders: &Orders, customer: &Customer) -> TypeQ13 {
    let mut o_h: HashMap<i32, i32> = (0..orders.9)
        .into_iter()
        .map(|i| {
            (
                i,
                orders.8[i as usize]
                    .find(&"special")
                    .map(|i| i as i32)
                    .unwrap_or(-1),
            )
        })
        .filter(|&(i, idx_special)| {
            idx_special == -1i32
                || orders.8[i as usize]
                    .rfind(&"requests")
                    .map(|i| i as i32)
                    .unwrap_or(-1)
                    < idx_special + 7i32
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<i32, i32>, (i, idx_special)| {
                acc[&orders.1[i as usize]] += 1i32;
                acc
            },
        );
    let mut c_h: HashMap<Record<(i32,)>, Record<(i32,)>> = (0..customer.8).into_iter().fold(
        HashMap::default(),
        |mut acc: HashMap<Record<(i32,)>, Record<(i32,)>>, i| {
            acc[&Record::new((if o_h.contains_key(&customer.0[i as usize]) {
                o_h[&customer.0[i as usize]]
            } else {
                0i32
            },))] += Record::new((1i32,));
            acc
        },
    );
    c_h.into_iter()
        .map(|(k, v)| (Record::new((k.0, v.0)), TRUE))
        .collect()
}

pub fn tpch_13_parallel(orders: &Orders, customer: &Customer) -> TypeQ13 {
    let mut o_h: HashMap<i32, i32> = (0..orders.9)
        .into_par_iter()
        .map(|i| {
            (
                i,
                orders.8[i as usize]
                    .find(&"special")
                    .map(|i| i as i32)
                    .unwrap_or(-1),
            )
        })
        .filter(|&(i, idx_special)| {
            idx_special == -1i32
                || orders.8[i as usize]
                    .rfind(&"requests")
                    .map(|i| i as i32)
                    .unwrap_or(-1)
                    < idx_special + 7i32
        })
        .fold(
            HashMap::default,
            |mut acc: HashMap<i32, i32>, (i, idx_special)| {
                acc[&orders.1[i as usize]] += 1i32;
                acc
            },
        )
        .sum();
    let mut c_h: HashMap<Record<(i32,)>, Record<(i32,)>> = (0..customer.8)
        .into_par_iter()
        .fold(
            HashMap::default,
            |mut acc: HashMap<Record<(i32,)>, Record<(i32,)>>, i| {
                acc[&Record::new((if o_h.contains_key(&customer.0[i as usize]) {
                    o_h[&customer.0[i as usize]]
                } else {
                    0i32
                },))] += Record::new((1i32,));
                acc
            },
        )
        .sum();
    c_h.into_par_iter()
        .map(|(k, v)| (Record::new((k.0, v.0)), TRUE))
        .collect()
}
