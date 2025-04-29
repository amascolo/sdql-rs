#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn q14_query(lineitem: &Lineitem, part: &Part) -> TypeQ14 {
    let mut p_h: HashMap<i32, Record<(i32,)>> = (0..part.9)
        .into_iter()
        .filter(|&i| part.4[i as usize].starts_with(&"PROMO"))
        .map(|i| (part.0[i as usize], Record::new((part.0[i as usize],))))
        .collect();
    let mut l_t: Record<(OrderedFloat<f64>, OrderedFloat<f64>)> = (0..lineitem.16)
        .into_iter()
        .filter(|&i| {
            date!(19950901) <= lineitem.10[i as usize] && lineitem.10[i as usize] < date!(19951001)
        })
        .map(|i| {
            Record::new((
                if p_h.contains_key(&lineitem.1[i as usize]) {
                    lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize])
                } else {
                    OrderedFloat(0f64)
                },
                lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]),
            ))
        })
        .sum();
    OrderedFloat(100f64) * l_t.0 / l_t.1
}

pub fn q14_query_rayon(lineitem: &Lineitem, part: &Part) -> TypeQ14 {
    let mut p_h: HashMap<i32, Record<(i32,)>> = (0..part.9)
        .into_par_iter()
        .filter(|&i| part.4[i as usize].starts_with(&"PROMO"))
        .map(|i| (part.0[i as usize], Record::new((part.0[i as usize],))))
        .collect();
    let mut l_t: Record<(OrderedFloat<f64>, OrderedFloat<f64>)> = (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| {
            date!(19950901) <= lineitem.10[i as usize] && lineitem.10[i as usize] < date!(19951001)
        })
        .map(|i| {
            Record::new((
                if p_h.contains_key(&lineitem.1[i as usize]) {
                    lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize])
                } else {
                    OrderedFloat(0f64)
                },
                lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]),
            ))
        })
        .sum();
    OrderedFloat(100f64) * l_t.0 / l_t.1
}
