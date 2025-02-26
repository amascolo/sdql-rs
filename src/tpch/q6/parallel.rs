use super::TypeQ6;
use crate::tpch::read::read_lineitems;
use crate::tpch::types::Lineitem;
use rayon::prelude::*;
use std::error::Error;

pub fn q6_rayon(sf: &str) -> Result<TypeQ6, Box<dyn Error>> {
    let lineitem = read_lineitems(&format!("datasets/tpch_datasets/SF_{sf}/lineitem.tbl"))?;
    Ok(q6_query_rayon(&lineitem))
}

pub fn q6_query_rayon(lineitem: &Lineitem) -> TypeQ6 {
    (0../* size */ lineitem.16)
        .into_par_iter()
        .map(|i| {
            if (0.05 <= /* discount */ lineitem.6[i])
                && (/* discount */lineitem.6[i] <= 0.07)
                && (/* quantity */lineitem.4[i] < 24f64)
                && (19940101 <= /* shipdate */ lineitem.10[i])
                && (/* shipdate */lineitem.10[i] < 19950101)
            {
                /* extendedprice */
                lineitem.5[i] * /* discount */ lineitem.6[i]
            } else {
                0f64
            }
        })
        .sum()
}

// note: this performed 25% faster
// pub fn q6_query_rayon(lineitem: &Lineitem) -> TypeQ6 {
//     (0../* size */ lineitem.16)
//         .into_par_iter()
//         .filter(|&i| {
//             (0.05 <= /* discount */ lineitem.6[i])
//                 && (/* discount */lineitem.6[i] <= 0.07)
//                 && (/* quantity */lineitem.4[i] < 24f64)
//                 && (19940101 <= /* shipdate */ lineitem.10[i])
//                 && (/* shipdate */lineitem.10[i] < 19950101)
//         })
//         .map(|i| /* extendedprice */ lineitem.5[i] * /* discount */ lineitem.6[i])
//         .sum()
// }
