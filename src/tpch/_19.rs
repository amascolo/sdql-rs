#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_19(lineitem: &Lineitem, part: &Part) -> TypeQ19 {
    let mut p_h: HashMap<i32, Record<(VarChar<10>, i32, VarChar<10>)>> = (0..part.9)
        .into_iter()
        .filter(|&i| {
            part.3[i as usize] == VarChar::from_str("Brand#12").unwrap()
                && (part.6[i as usize] == VarChar::from_str("SM CASE").unwrap()
                    || part.6[i as usize] == VarChar::from_str("SM BOX").unwrap()
                    || part.6[i as usize] == VarChar::from_str("SM PACK").unwrap()
                    || part.6[i as usize] == VarChar::from_str("SM PKG").unwrap())
                && 1i32 <= part.5[i as usize]
                && part.5[i as usize] <= 5i32
                || part.3[i as usize] == VarChar::from_str("Brand#23").unwrap()
                    && (part.6[i as usize] == VarChar::from_str("MED BAG").unwrap()
                        || part.6[i as usize] == VarChar::from_str("MED BOX").unwrap()
                        || part.6[i as usize] == VarChar::from_str("MED PACK").unwrap()
                        || part.6[i as usize] == VarChar::from_str("MED PKG").unwrap())
                    && 1i32 <= part.5[i as usize]
                    && part.5[i as usize] <= 10i32
                || part.3[i as usize] == VarChar::from_str("Brand#34").unwrap()
                    && (part.6[i as usize] == VarChar::from_str("LG CASE").unwrap()
                        || part.6[i as usize] == VarChar::from_str("LG BOX").unwrap()
                        || part.6[i as usize] == VarChar::from_str("LG PACK").unwrap()
                        || part.6[i as usize] == VarChar::from_str("LG PKG").unwrap())
                    && 1i32 <= part.5[i as usize]
                    && part.5[i as usize] <= 15i32
        })
        .map(|i| {
            (
                part.0[i as usize],
                Record::new((part.3[i as usize], part.5[i as usize], part.6[i as usize])),
            )
        })
        .collect();
    let mut res: OrderedFloat<f64> = (0..lineitem.16)
        .into_iter()
        .map(|i| (i, p_h[&lineitem.1[i as usize]].0))
        .filter(|&(i, p_brand)| {
            p_h.contains_key(&lineitem.1[i as usize])
                && (lineitem.14[i as usize] == VarChar::from_str("AIR").unwrap()
                    || lineitem.14[i as usize] == VarChar::from_str("AIR REG").unwrap())
                && lineitem.13[i as usize] == VarChar::from_str("DELIVER IN PERSON").unwrap()
                && (p_brand == VarChar::from_str("Brand#12").unwrap()
                    && OrderedFloat(1f64) <= lineitem.4[i as usize]
                    && lineitem.4[i as usize] <= OrderedFloat(11f64)
                    || p_brand == VarChar::from_str("Brand#23").unwrap()
                        && OrderedFloat(10f64) <= lineitem.4[i as usize]
                        && lineitem.4[i as usize] <= OrderedFloat(20f64)
                    || p_brand == VarChar::from_str("Brand#34").unwrap()
                        && OrderedFloat(20f64) <= lineitem.4[i as usize]
                        && lineitem.4[i as usize] <= OrderedFloat(30f64))
        })
        .map(|(i, p_brand)| lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]))
        .sum();
    HashMap::from([(Record::new((res,)), TRUE)])
}

pub fn tpch_19_parallel(lineitem: &Lineitem, part: &Part) -> TypeQ19 {
    let mut p_h: HashMap<i32, Record<(VarChar<10>, i32, VarChar<10>)>> = (0..part.9)
        .into_par_iter()
        .filter(|&i| {
            part.3[i as usize] == VarChar::from_str("Brand#12").unwrap()
                && (part.6[i as usize] == VarChar::from_str("SM CASE").unwrap()
                    || part.6[i as usize] == VarChar::from_str("SM BOX").unwrap()
                    || part.6[i as usize] == VarChar::from_str("SM PACK").unwrap()
                    || part.6[i as usize] == VarChar::from_str("SM PKG").unwrap())
                && 1i32 <= part.5[i as usize]
                && part.5[i as usize] <= 5i32
                || part.3[i as usize] == VarChar::from_str("Brand#23").unwrap()
                    && (part.6[i as usize] == VarChar::from_str("MED BAG").unwrap()
                        || part.6[i as usize] == VarChar::from_str("MED BOX").unwrap()
                        || part.6[i as usize] == VarChar::from_str("MED PACK").unwrap()
                        || part.6[i as usize] == VarChar::from_str("MED PKG").unwrap())
                    && 1i32 <= part.5[i as usize]
                    && part.5[i as usize] <= 10i32
                || part.3[i as usize] == VarChar::from_str("Brand#34").unwrap()
                    && (part.6[i as usize] == VarChar::from_str("LG CASE").unwrap()
                        || part.6[i as usize] == VarChar::from_str("LG BOX").unwrap()
                        || part.6[i as usize] == VarChar::from_str("LG PACK").unwrap()
                        || part.6[i as usize] == VarChar::from_str("LG PKG").unwrap())
                    && 1i32 <= part.5[i as usize]
                    && part.5[i as usize] <= 15i32
        })
        .map(|i| {
            (
                part.0[i as usize],
                Record::new((part.3[i as usize], part.5[i as usize], part.6[i as usize])),
            )
        })
        .collect();
    let mut res: OrderedFloat<f64> = (0..lineitem.16)
        .into_par_iter()
        .map(|i| (i, p_h[&lineitem.1[i as usize]].0))
        .filter(|&(i, p_brand)| {
            p_h.contains_key(&lineitem.1[i as usize])
                && (lineitem.14[i as usize] == VarChar::from_str("AIR").unwrap()
                    || lineitem.14[i as usize] == VarChar::from_str("AIR REG").unwrap())
                && lineitem.13[i as usize] == VarChar::from_str("DELIVER IN PERSON").unwrap()
                && (p_brand == VarChar::from_str("Brand#12").unwrap()
                    && OrderedFloat(1f64) <= lineitem.4[i as usize]
                    && lineitem.4[i as usize] <= OrderedFloat(11f64)
                    || p_brand == VarChar::from_str("Brand#23").unwrap()
                        && OrderedFloat(10f64) <= lineitem.4[i as usize]
                        && lineitem.4[i as usize] <= OrderedFloat(20f64)
                    || p_brand == VarChar::from_str("Brand#34").unwrap()
                        && OrderedFloat(20f64) <= lineitem.4[i as usize]
                        && lineitem.4[i as usize] <= OrderedFloat(30f64))
        })
        .map(|(i, p_brand)| lineitem.5[i as usize] * (OrderedFloat(1f64) - lineitem.6[i as usize]))
        .sum();
    HashMap::from([(Record::new((res,)), TRUE)])
}
