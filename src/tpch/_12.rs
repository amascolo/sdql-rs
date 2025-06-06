#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_12(orders: &Orders, lineitem: &Lineitem) -> TypeQ12 {
    let mut l_h: HashMap<i32, HashMap<VarChar<10>, i32>> = (0..lineitem.16)
        .into_iter()
        .filter(|&i| {
            (lineitem.14[i as usize] == VarChar::from_str("MAIL").unwrap()
                || lineitem.14[i as usize] == VarChar::from_str("SHIP").unwrap())
                && date!(19940101) <= lineitem.12[i as usize]
                && lineitem.12[i as usize] < date!(19950101)
                && lineitem.10[i as usize] < lineitem.11[i as usize]
                && lineitem.11[i as usize] < lineitem.12[i as usize]
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<i32, HashMap<VarChar<10>, i32>>, i| {
                acc[&lineitem.0[i as usize]][&lineitem.14[i as usize]] += 1i32;
                acc
            },
        );
    let mut o_h: HashMap<Record<(VarChar<10>,)>, Record<(i32, i32)>> = (0..orders.9)
        .into_iter()
        // TODO update code generator
        // .filter_map(|i| l_h.remove(&orders.0[i as usize]).map(|inner| (i, inner)))
        // .flat_map(|(i, inner)| {
        //     inner
        //         .into_iter()
        //         .map(move |(l_shipmode, c)| (i, l_shipmode, c))
        // })
        .filter(|&i| l_h.contains_key(&orders.0[i as usize]))
        .flat_map(|i| {
            l_h[&orders.0[i as usize]]
                .iter()
                .map(move |(&l_shipmode, &c)| (i, l_shipmode, c))
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<Record<(VarChar<10>,)>, Record<(i32, i32)>>, (i, l_shipmode, c)| {
                acc[&Record::new((l_shipmode,))] += Record::new((
                    if orders.5[i as usize] == VarChar::from_str("1-URGENT").unwrap()
                        || orders.5[i as usize] == VarChar::from_str("2-HIGH").unwrap()
                    {
                        c
                    } else {
                        0i32
                    },
                    if orders.5[i as usize] != VarChar::from_str("1-URGENT").unwrap()
                        && orders.5[i as usize] != VarChar::from_str("2-HIGH").unwrap()
                    {
                        c
                    } else {
                        0i32
                    },
                ));
                acc
            },
        );
    o_h.into_iter()
        .map(|(k, v)| (Record::new((k.0, v.0, v.1)), TRUE))
        .collect()
}

pub fn tpch_12_parallel(orders: &Orders, lineitem: &Lineitem) -> TypeQ12 {
    let mut l_h: HashMap<i32, HashMap<VarChar<10>, i32>> = (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| {
            (lineitem.14[i as usize] == VarChar::from_str("MAIL").unwrap()
                || lineitem.14[i as usize] == VarChar::from_str("SHIP").unwrap())
                && date!(19940101) <= lineitem.12[i as usize]
                && lineitem.12[i as usize] < date!(19950101)
                && lineitem.10[i as usize] < lineitem.11[i as usize]
                && lineitem.11[i as usize] < lineitem.12[i as usize]
        })
        .fold(
            HashMap::default,
            |mut acc: HashMap<i32, HashMap<VarChar<10>, i32>>, i| {
                acc[&lineitem.0[i as usize]][&lineitem.14[i as usize]] += 1i32;
                acc
            },
        )
        .sum();
    let mut o_h: HashMap<Record<(VarChar<10>,)>, Record<(i32, i32)>> = (0..orders.9)
        // TODO update code generator
        // .into_iter()
        // .filter_map(|i| l_h.remove(&orders.0[i as usize]).map(|inner| (i, inner)))
        // .par_bridge()
        // .flat_map_iter(|(i, inner)| {
        //     inner
        //         .into_iter()
        //         .map(move |(l_shipmode, c)| (i, l_shipmode, c))
        // })
        .into_par_iter()
        .filter(|&i| l_h.contains_key(&orders.0[i as usize]))
        .flat_map_iter(|i| {
            l_h[&orders.0[i as usize]]
                .iter()
                .map(move |(&l_shipmode, &c)| (i, l_shipmode, c))
        })
        // note: we can parallelise on the inner map too, but the overhead makes it slower (on SF=1)
        // .flat_map(|i| {
        //     l_h[&orders.0[i as usize]]
        //         .par_iter()
        //         .map(move |(&l_shipmode, &c)| (i, l_shipmode, c))
        // })
        .fold(
            HashMap::default,
            |mut acc: HashMap<Record<(VarChar<10>,)>, Record<(i32, i32)>>, (i, l_shipmode, c)| {
                acc[&Record::new((l_shipmode,))] += Record::new((
                    if orders.5[i as usize] == VarChar::from_str("1-URGENT").unwrap()
                        || orders.5[i as usize] == VarChar::from_str("2-HIGH").unwrap()
                    {
                        c
                    } else {
                        0i32
                    },
                    if orders.5[i as usize] != VarChar::from_str("1-URGENT").unwrap()
                        && orders.5[i as usize] != VarChar::from_str("2-HIGH").unwrap()
                    {
                        c
                    } else {
                        0i32
                    },
                ));
                acc
            },
        )
        .sum();
    o_h.into_par_iter()
        .map(|(k, v)| (Record::new((k.0, v.0, v.1)), TRUE))
        .collect()
}
