#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_16(partsupp: &Partsupp, part: &Part, supplier: &Supplier) -> TypeQ16 {
    let mut p_h: HashMap<i32, Record<(VarChar<10>, VarChar<25>, i32)>> = (0..part.9)
        .into_iter()
        .map(|i| (i, !part.4[i as usize].starts_with(&"MEDIUM POLISHED")))
        .filter(|&(i, cond)| {
            part.3[i as usize] != VarChar::from_str("Brand#45").unwrap()
                && cond
                && (part.5[i as usize] == 49i32
                    || part.5[i as usize] == 14i32
                    || part.5[i as usize] == 23i32
                    || part.5[i as usize] == 45i32
                    || part.5[i as usize] == 19i32
                    || part.5[i as usize] == 3i32
                    || part.5[i as usize] == 36i32
                    || part.5[i as usize] == 9i32)
        })
        .map(|(i, cond)| {
            (
                part.0[i as usize],
                Record::new((part.3[i as usize], part.4[i as usize], part.5[i as usize])),
            )
        })
        .collect();
    let mut s_h: HashMap<i32, Record<(i32,)>> = (0..supplier.7)
        .into_iter()
        .map(|i| {
            (
                i,
                supplier.6[i as usize]
                    .find(&"Customer")
                    .map(|i| i as i32)
                    .unwrap_or(-1),
            )
        })
        .map(|(i, idx_customer)| {
            (
                i,
                idx_customer,
                supplier.6[i as usize]
                    .find(&"Complaints")
                    .map(|i| i as i32)
                    .unwrap_or(-1),
            )
        })
        .filter(|&(i, idx_customer, idx_complaints)| {
            idx_customer != -1i32 && idx_customer + 8i32 <= idx_complaints
        })
        .map(|(i, idx_customer, idx_complaints)| {
            (
                supplier.0[i as usize],
                Record::new((supplier.0[i as usize],)),
            )
        })
        .collect();
    let mut ps_h: HashMap<Record<(VarChar<10>, VarChar<25>, i32)>, HashMap<i32, i32>> = (0
        ..partsupp.5)
        .into_iter()
        .filter(|&i| {
            p_h.contains_key(&partsupp.0[i as usize]) && !s_h.contains_key(&partsupp.1[i as usize])
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<Record<(VarChar<10>, VarChar<25>, i32)>, HashMap<i32, i32>>, i| {
                acc[&Record::new((
                    p_h[&partsupp.0[i as usize]].0,
                    p_h[&partsupp.0[i as usize]].1,
                    p_h[&partsupp.0[i as usize]].2,
                ))][&partsupp.1[i as usize]] += 1i32;
                acc
            },
        );
    ps_h.into_iter()
        .map(|(k, v_hashmap)| {
            (
                Record::new((k.0, k.1, k.2, Record::new((v_hashmap.len() as i32,)).0)),
                TRUE,
            )
        })
        .collect()
}

pub fn tpch_16_parallel(partsupp: &Partsupp, part: &Part, supplier: &Supplier) -> TypeQ16 {
    let mut p_h: HashMap<i32, Record<(VarChar<10>, VarChar<25>, i32)>> = (0..part.9)
        .into_par_iter()
        .map(|i| (i, !part.4[i as usize].starts_with(&"MEDIUM POLISHED")))
        .filter(|&(i, cond)| {
            part.3[i as usize] != VarChar::from_str("Brand#45").unwrap()
                && cond
                && (part.5[i as usize] == 49i32
                    || part.5[i as usize] == 14i32
                    || part.5[i as usize] == 23i32
                    || part.5[i as usize] == 45i32
                    || part.5[i as usize] == 19i32
                    || part.5[i as usize] == 3i32
                    || part.5[i as usize] == 36i32
                    || part.5[i as usize] == 9i32)
        })
        .map(|(i, cond)| {
            (
                part.0[i as usize],
                Record::new((part.3[i as usize], part.4[i as usize], part.5[i as usize])),
            )
        })
        .collect();
    let mut s_h: HashMap<i32, Record<(i32,)>> = (0..supplier.7)
        .into_par_iter()
        .map(|i| {
            (
                i,
                supplier.6[i as usize]
                    .find(&"Customer")
                    .map(|i| i as i32)
                    .unwrap_or(-1),
            )
        })
        .map(|(i, idx_customer)| {
            (
                i,
                idx_customer,
                supplier.6[i as usize]
                    .find(&"Complaints")
                    .map(|i| i as i32)
                    .unwrap_or(-1),
            )
        })
        .filter(|&(i, idx_customer, idx_complaints)| {
            idx_customer != -1i32 && idx_customer + 8i32 <= idx_complaints
        })
        .map(|(i, idx_customer, idx_complaints)| {
            (
                supplier.0[i as usize],
                Record::new((supplier.0[i as usize],)),
            )
        })
        .collect();
    let mut ps_h: HashMap<Record<(VarChar<10>, VarChar<25>, i32)>, HashMap<i32, i32>> = (0
        ..partsupp.5)
        .into_par_iter()
        .filter(|&i| {
            p_h.contains_key(&partsupp.0[i as usize]) && !s_h.contains_key(&partsupp.1[i as usize])
        })
        .fold(
            HashMap::default,
            |mut acc: HashMap<Record<(VarChar<10>, VarChar<25>, i32)>, HashMap<i32, i32>>, i| {
                acc[&Record::new((
                    p_h[&partsupp.0[i as usize]].0,
                    p_h[&partsupp.0[i as usize]].1,
                    p_h[&partsupp.0[i as usize]].2,
                ))][&partsupp.1[i as usize]] += 1i32;
                acc
            },
        )
        .sum();
    ps_h.into_par_iter()
        .map(|(k, v_hashmap)| {
            (
                Record::new((k.0, k.1, k.2, Record::new((v_hashmap.len() as i32,)).0)),
                TRUE,
            )
        })
        .collect()
}
