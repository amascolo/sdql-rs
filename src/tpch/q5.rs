#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn q5_query(
    customer: &Customer,
    orders: &Orders,
    lineitem: &Lineitem,
    supplier: &Supplier,
    nation: &Nation,
    region: &Region,
) -> TypeQ5 {
    let mut r_h: HashMap<i32, Record<(i32,)>> = (0..region.3)
        .into_iter()
        .filter(|&i| region.1[i as usize] == VarChar::from_str("ASIA").unwrap())
        .map(|i| (region.0[i as usize], Record::new((region.0[i as usize],))))
        .collect();
    let mut n_h: HashMap<i32, VarChar<25>> = (0..nation.4)
        .into_iter()
        .filter(|&i| r_h.contains_key(&nation.2[i as usize]))
        .map(|i| (nation.0[i as usize], nation.1[i as usize]))
        .collect();
    let mut c_h: HashMap<i32, Record<(VarChar<25>, i32)>> = (0..customer.8)
        .into_iter()
        .filter(|&i| n_h.contains_key(&customer.3[i as usize]))
        .map(|i| {
            (
                customer.0[i as usize],
                Record::new((n_h[&customer.3[i as usize]], customer.3[i as usize])),
            )
        })
        .collect();
    let mut o_h: HashMap<i32, Record<(VarChar<25>, i32)>> = (0..orders.9)
        .into_iter()
        .filter(|&i| {
            orders.4[i as usize] < date!(19950101)
                && date!(19940101) <= orders.4[i as usize]
                && c_h.contains_key(&orders.1[i as usize])
        })
        .map(|i| {
            (
                orders.0[i as usize],
                Record::new((c_h[&orders.1[i as usize]].0, c_h[&orders.1[i as usize]].1)),
            )
        })
        .collect();
    let mut s_h: HashMap<Record<(i32, i32)>, i32> = (0..supplier.7)
        .into_iter()
        .map(|i| {
            (
                Record::new((supplier.0[i as usize], supplier.3[i as usize])),
                1i32,
            )
        })
        .collect();
    let mut l_h: HashMap<VarChar<25>, OrderedFloat<f64>> = (0..lineitem.16)
        .into_iter()
        .filter(|&i| {
            o_h.contains_key(&lineitem.0[i as usize])
                && s_h.contains_key(&Record::new((
                    lineitem.2[i as usize],
                    o_h[&lineitem.0[i as usize]].1,
                )))
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<VarChar<25>, OrderedFloat<f64>>, i| {
                acc[&o_h[&lineitem.0[i as usize]].0] +=
                    lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]);
                acc
            },
        );
    l_h.into_iter()
        .map(|(k, v)| (Record::new((k, v)), TRUE))
        .collect()
}

pub fn q5_query_rayon(
    customer: &Customer,
    orders: &Orders,
    lineitem: &Lineitem,
    supplier: &Supplier,
    nation: &Nation,
    region: &Region,
) -> TypeQ5 {
    let mut r_h: HashMap<i32, Record<(i32,)>> = (0..region.3)
        .into_par_iter()
        .filter(|&i| region.1[i as usize] == VarChar::from_str("ASIA").unwrap())
        .map(|i| (region.0[i as usize], Record::new((region.0[i as usize],))))
        .collect();
    let mut n_h: HashMap<i32, VarChar<25>> = (0..nation.4)
        .into_par_iter()
        .filter(|&i| r_h.contains_key(&nation.2[i as usize]))
        .map(|i| (nation.0[i as usize], nation.1[i as usize]))
        .collect();
    let mut c_h: HashMap<i32, Record<(VarChar<25>, i32)>> = (0..customer.8)
        .into_par_iter()
        .filter(|&i| n_h.contains_key(&customer.3[i as usize]))
        .map(|i| {
            (
                customer.0[i as usize],
                Record::new((n_h[&customer.3[i as usize]], customer.3[i as usize])),
            )
        })
        .collect();
    let mut o_h: HashMap<i32, Record<(VarChar<25>, i32)>> = (0..orders.9)
        .into_par_iter()
        .filter(|&i| {
            orders.4[i as usize] < date!(19950101)
                && date!(19940101) <= orders.4[i as usize]
                && c_h.contains_key(&orders.1[i as usize])
        })
        .map(|i| {
            (
                orders.0[i as usize],
                Record::new((c_h[&orders.1[i as usize]].0, c_h[&orders.1[i as usize]].1)),
            )
        })
        .collect();
    let mut s_h: HashMap<Record<(i32, i32)>, i32> = (0..supplier.7)
        .into_par_iter()
        .map(|i| {
            (
                Record::new((supplier.0[i as usize], supplier.3[i as usize])),
                1i32,
            )
        })
        .collect();
    let mut l_h: HashMap<VarChar<25>, OrderedFloat<f64>> = (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| {
            o_h.contains_key(&lineitem.0[i as usize])
                && s_h.contains_key(&Record::new((
                    lineitem.2[i as usize],
                    o_h[&lineitem.0[i as usize]].1,
                )))
        })
        .fold(
            HashMap::default,
            |mut acc: HashMap<VarChar<25>, OrderedFloat<f64>>, i| {
                acc[&o_h[&lineitem.0[i as usize]].0] +=
                    lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]);
                acc
            },
        )
        .sum();
    l_h.into_par_iter()
        .map(|(k, v)| (Record::new((k, v)), TRUE))
        .collect()
}
