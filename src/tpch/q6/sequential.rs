use super::TypeQ6;
use crate::tpch::read::read_lineitems;
use crate::tpch::types::Lineitem;
use std::error::Error;

pub fn q6(sf: &str) -> Result<TypeQ6, Box<dyn Error>> {
    let lineitem = read_lineitems(&format!("datasets/tpch_datasets/SF_{sf}/lineitem.tbl"))?;
    Ok(q6_query(&lineitem))
}

pub fn q6_query(lineitem: &Lineitem) -> TypeQ6 {
    (0../* size */ lineitem.16)
        .filter(|&i| {
            (0.05 <= /* discount */ lineitem.6[i])
                && (/* discount */lineitem.6[i] <= 0.07)
                && (/* quantity */lineitem.4[i] < 24f64)
                && (19940101 <= /* shipdate */ lineitem.10[i])
                && (/* shipdate */lineitem.10[i] < 19950101)
        })
        .map(|i| /* extendedprice */ lineitem.5[i] * /* discount */ lineitem.6[i])
        .sum()
}
