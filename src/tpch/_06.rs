#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_06(lineitem: &Lineitem) -> TypeQ6 {
    (0..lineitem.16)
        .into_iter()
        .filter(|&i| {
            OrderedFloat(0.05f64) <= lineitem.6[i as usize]
                && lineitem.6[i as usize] <= OrderedFloat(0.07f64)
                && lineitem.4[i as usize] < OrderedFloat(24f64)
                && date!(19940101) <= lineitem.10[i as usize]
                && lineitem.10[i as usize] < date!(19950101)
        })
        .map(|i| lineitem.5[i as usize] * lineitem.6[i as usize])
        .sum()
}

pub fn tpch_06_parallel(lineitem: &Lineitem) -> TypeQ6 {
    (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| {
            OrderedFloat(0.05f64) <= lineitem.6[i as usize]
                && lineitem.6[i as usize] <= OrderedFloat(0.07f64)
                && lineitem.4[i as usize] < OrderedFloat(24f64)
                && date!(19940101) <= lineitem.10[i as usize]
                && lineitem.10[i as usize] < date!(19950101)
        })
        .map(|i| lineitem.5[i as usize] * lineitem.6[i as usize])
        .sum()
}
