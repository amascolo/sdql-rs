use super::TypeQ6;
use super::{_19940101, _19950101};
use crate::tpch::read::read_lineitems;
use crate::tpch::types::Lineitem;
use sdql_runtime::OrderedFloat;
use std::error::Error;

pub fn q6(sf: &str) -> Result<TypeQ6, Box<dyn Error>> {
    let lineitem = read_lineitems()(&format!("datasets/tpch_datasets/SF_{sf}/lineitem.tbl"))?;
    Ok(q6_query(&lineitem))
}

pub fn q6_query(lineitem: &Lineitem) -> TypeQ6 {
    (0../* size */ lineitem.16)
        .map(|i| {
            if (OrderedFloat(0.05) <= /* discount */ lineitem.6[i])
                && (/* discount */lineitem.6[i] <= OrderedFloat(0.07))
                && (/* quantity */lineitem.4[i] < OrderedFloat(24f64))
                && (_19940101 <= /* shipdate */ lineitem.10[i])
                && (/* shipdate */lineitem.10[i] < _19950101)
            {
                /* extendedprice */
                lineitem.5[i] * /* discount */ lineitem.6[i]
            } else {
                OrderedFloat(0f64)
            }
        })
        .sum()
}
