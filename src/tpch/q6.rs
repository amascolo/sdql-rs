#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn q6_query(lineitem: &Lineitem) -> TypeQ6 {
    (0../* size */ lineitem.16)
        .map(|i| {
            if (OrderedFloat(0.05) <= /* discount */ lineitem.6[i])
                && (/* discount */lineitem.6[i] <= OrderedFloat(0.07))
                && (/* quantity */lineitem.4[i] < OrderedFloat(24f64))
                && (date!(19940101) <= /* shipdate */ lineitem.10[i])
                && (/* shipdate */lineitem.10[i] < date!(19950101))
            {
                /* extendedprice */
                lineitem.5[i] * /* discount */ lineitem.6[i]
            } else {
                OrderedFloat(0f64)
            }
        })
        .sum()
}

pub fn q6_query_rayon(lineitem: &Lineitem) -> TypeQ6 {
    (0../* size */ lineitem.16)
        .into_par_iter()
        .map(|i| {
            if (OrderedFloat(0.05) <= /* discount */ lineitem.6[i])
                && (/* discount */lineitem.6[i] <= OrderedFloat(0.07))
                && (/* quantity */lineitem.4[i] < OrderedFloat(24f64))
                && (date!(19940101) <= /* shipdate */ lineitem.10[i])
                && (/* shipdate */lineitem.10[i] < date!(19950101))
            {
                /* extendedprice */
                lineitem.5[i] * /* discount */ lineitem.6[i]
            } else {
                OrderedFloat(0f64)
            }
        })
        .sum()
}
