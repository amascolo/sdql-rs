use super::{TypeQ6, _19940101, _19950101};
use crate::tpch::read::read_lineitems;
use crate::tpch::types::Lineitem;
use rayon::prelude::*;
use sdql_runtime::Real;
use std::error::Error;

pub fn q6_rayon(sf: &str) -> Result<TypeQ6, Box<dyn Error>> {
    let lineitem = read_lineitems()(&format!("datasets/tpch_datasets/SF_{sf}/lineitem.tbl"))?;
    Ok(q6_query_rayon(&lineitem))
}

pub fn q6_query_rayon(lineitem: &Lineitem) -> TypeQ6 {
    (0../* size */ lineitem.16)
        .into_par_iter()
        .map(|i| {
            if (Real::new(0.05) <= /* discount */ lineitem.6[i])
                && (/* discount */lineitem.6[i] <= Real::new(0.07))
                && (/* quantity */lineitem.4[i] < Real::new(24f64))
                && (_19940101 <= /* shipdate */ lineitem.10[i])
                && (/* shipdate */lineitem.10[i] < _19950101)
            {
                /* extendedprice */
                lineitem.5[i] * /* discount */ lineitem.6[i]
            } else {
                Real::new(0f64)
            }
        })
        .sum()
}
