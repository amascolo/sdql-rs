#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn q20_query(
    supplier: &Supplier,
    nation: &Nation,
    part: &Part,
    partsupp: &Partsupp,
    lineitem: &Lineitem,
) -> TypeQ20 {
    let mut p_h: HashMap<i32, Record<(i32,)>> = (0..part.9)
        .into_iter()
        .filter(|&i| part.1[i as usize].starts_with(&"forest"))
        .map(|i| (part.0[i as usize], Record::new((part.0[i as usize],))))
        .collect();
    let mut n_h: HashMap<i32, Record<(i32,)>> = (0..nation.4)
        .into_iter()
        .filter(|&i| nation.1[i as usize] == VarChar::from_str("CANADA").unwrap())
        .map(|i| (nation.0[i as usize], Record::new((nation.0[i as usize],))))
        .collect();
    let mut s_h: HashMap<i32, Record<(i32,)>> = (0..supplier.7)
        .into_iter()
        .filter(|&i| n_h.contains_key(&supplier.3[i as usize]))
        .map(|i| {
            (
                supplier.0[i as usize],
                Record::new((supplier.0[i as usize],)),
            )
        })
        .collect();
    let mut l_h: HashMap<Record<(i32, i32)>, OrderedFloat<f64>> = (0..lineitem.16)
        .into_iter()
        .filter(|&i| {
            date!(19940101) <= lineitem.10[i as usize]
                && lineitem.10[i as usize] < date!(19950101)
                && p_h.contains_key(&lineitem.1[i as usize])
                && s_h.contains_key(&lineitem.2[i as usize])
        })
        .map(|i| {
            (
                Record::new((lineitem.1[i as usize], lineitem.2[i as usize])),
                OrderedFloat(0.5f64) * lineitem.4[i as usize],
            )
        })
        .collect();
    let mut ps_h: HashMap<i32, Record<(i32,)>> = (0..partsupp.5)
        .into_iter()
        .map(|i| {
            (
                i,
                Record::new((partsupp.0[i as usize], partsupp.1[i as usize])),
            )
        })
        .filter(|&(i, key)| l_h.contains_key(&key) && l_h[&key] < partsupp.2[i as usize])
        .map(|(i, key)| {
            (
                partsupp.1[i as usize],
                Record::new((partsupp.1[i as usize],)),
            )
        })
        .collect();
    (0..supplier.7)
        .into_iter()
        .filter(|&i| ps_h.contains_key(&supplier.0[i as usize]))
        .map(|i| {
            (
                Record::new((supplier.1[i as usize], supplier.2[i as usize])),
                TRUE,
            )
        })
        .collect()
}

pub fn q20_query_rayon(
    supplier: &Supplier,
    nation: &Nation,
    part: &Part,
    partsupp: &Partsupp,
    lineitem: &Lineitem,
) -> TypeQ20 {
    let mut p_h: HashMap<i32, Record<(i32,)>> = (0..part.9)
        .into_par_iter()
        .filter(|&i| part.1[i as usize].starts_with(&"forest"))
        .map(|i| (part.0[i as usize], Record::new((part.0[i as usize],))))
        .collect();
    let mut n_h: HashMap<i32, Record<(i32,)>> = (0..nation.4)
        .into_par_iter()
        .filter(|&i| nation.1[i as usize] == VarChar::from_str("CANADA").unwrap())
        .map(|i| (nation.0[i as usize], Record::new((nation.0[i as usize],))))
        .collect();
    let mut s_h: HashMap<i32, Record<(i32,)>> = (0..supplier.7)
        .into_par_iter()
        .filter(|&i| n_h.contains_key(&supplier.3[i as usize]))
        .map(|i| {
            (
                supplier.0[i as usize],
                Record::new((supplier.0[i as usize],)),
            )
        })
        .collect();
    let mut l_h: HashMap<Record<(i32, i32)>, OrderedFloat<f64>> = (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| {
            date!(19940101) <= lineitem.10[i as usize]
                && lineitem.10[i as usize] < date!(19950101)
                && p_h.contains_key(&lineitem.1[i as usize])
                && s_h.contains_key(&lineitem.2[i as usize])
        })
        .map(|i| {
            (
                Record::new((lineitem.1[i as usize], lineitem.2[i as usize])),
                OrderedFloat(0.5f64) * lineitem.4[i as usize],
            )
        })
        .collect();
    let mut ps_h: HashMap<i32, Record<(i32,)>> = (0..partsupp.5)
        .into_par_iter()
        .map(|i| {
            (
                i,
                Record::new((partsupp.0[i as usize], partsupp.1[i as usize])),
            )
        })
        .filter(|&(i, key)| l_h.contains_key(&key) && l_h[&key] < partsupp.2[i as usize])
        .map(|(i, key)| {
            (
                partsupp.1[i as usize],
                Record::new((partsupp.1[i as usize],)),
            )
        })
        .collect();
    (0..supplier.7)
        .into_par_iter()
        .filter(|&i| ps_h.contains_key(&supplier.0[i as usize]))
        .map(|i| {
            (
                Record::new((supplier.1[i as usize], supplier.2[i as usize])),
                TRUE,
            )
        })
        .collect()
}
