#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_17(lineitem: &Lineitem, part: &Part) -> TypeQ17 {
    let mut p_h: HashMap<i32, Record<(i32,)>> = (0..part.9)
        .into_iter()
        .filter(|&i| {
            part.3[i as usize] == VarChar::from_str("Brand#23").unwrap()
                && part.6[i as usize] == VarChar::from_str("MED BOX").unwrap()
        })
        .map(|i| (part.0[i as usize], Record::new((part.0[i as usize],))))
        .collect();
    let mut l_h: HashMap<i32, Record<(OrderedFloat<f64>, OrderedFloat<f64>)>> = (0..lineitem.16)
        .into_iter()
        .filter(|&i| p_h.contains_key(&lineitem.1[i as usize]))
        .fold(
            HashMap::default(),
            |mut acc: HashMap<i32, Record<(OrderedFloat<f64>, OrderedFloat<f64>)>>, i| {
                acc[&lineitem.1[i as usize]] +=
                    Record::new((lineitem.4[i as usize], OrderedFloat(1f64)));
                acc
            },
        );
    let mut tot: OrderedFloat<f64> = (0..lineitem.16)
        .into_iter()
        .map(|i| {
            (
                i,
                OrderedFloat(0.2f64) * l_h[&lineitem.1[i as usize]].0
                    / l_h[&lineitem.1[i as usize]].1,
            )
        })
        .filter(|&(i, avg)| {
            l_h.contains_key(&lineitem.1[i as usize]) && lineitem.4[i as usize] < avg
        })
        .map(|(i, avg)| lineitem.5[i as usize])
        .sum();
    tot / OrderedFloat(7f64)
}

pub fn tpch_17_parallel(lineitem: &Lineitem, part: &Part) -> TypeQ17 {
    let mut p_h: HashMap<i32, Record<(i32,)>> = (0..part.9)
        .into_par_iter()
        .filter(|&i| {
            part.3[i as usize] == VarChar::from_str("Brand#23").unwrap()
                && part.6[i as usize] == VarChar::from_str("MED BOX").unwrap()
        })
        .map(|i| (part.0[i as usize], Record::new((part.0[i as usize],))))
        .collect();
    let mut l_h: HashMap<i32, Record<(OrderedFloat<f64>, OrderedFloat<f64>)>> = (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| p_h.contains_key(&lineitem.1[i as usize]))
        .fold(
            HashMap::default,
            |mut acc: HashMap<i32, Record<(OrderedFloat<f64>, OrderedFloat<f64>)>>, i| {
                acc[&lineitem.1[i as usize]] +=
                    Record::new((lineitem.4[i as usize], OrderedFloat(1f64)));
                acc
            },
        )
        .sum();
    let mut tot: OrderedFloat<f64> = (0..lineitem.16)
        .into_par_iter()
        .map(|i| {
            (
                i,
                OrderedFloat(0.2f64) * l_h[&lineitem.1[i as usize]].0
                    / l_h[&lineitem.1[i as usize]].1,
            )
        })
        .filter(|&(i, avg)| {
            l_h.contains_key(&lineitem.1[i as usize]) && lineitem.4[i as usize] < avg
        })
        .map(|(i, avg)| lineitem.5[i as usize])
        .sum();
    tot / OrderedFloat(7f64)
}
