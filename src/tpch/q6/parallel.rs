use super::TypeQ6;
use crate::tpch::read::read_lineitems;
use crate::tpch::types::Lineitem;
use rayon::prelude::*;
use std::error::Error;

pub fn q6_rayon() -> Result<TypeQ6, Box<dyn Error>> {
    let lineitem = read_lineitems("datasets/tpch_datasets/SF_1/lineitem.tbl")?;
    Ok(q6_query_rayon(&lineitem))
}

pub fn q6_query_rayon(lineitem: &Lineitem) -> TypeQ6 {
    (0../* size */ lineitem.16)
        .into_par_iter()
        .filter(|&i| 0.05 <= /* discount */ lineitem.6[i])
        .filter(|&i| /* discount */ lineitem.6[i] <= 0.07)
        .filter(|&i| /* quantity */ lineitem.4[i] <= 24f64)
        .filter(|&i| 19940101 <= /* shipdate */ lineitem.10[i])
        .filter(|&i| /* shipdate */ lineitem.10[i] < 19950101)
        .map(|i| /* extendedprice */ lineitem.5[i] * /* discount */ lineitem.6[i])
        .sum()
}
