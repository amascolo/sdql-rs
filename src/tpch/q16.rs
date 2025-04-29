#![allow(unused_mut, unused_variables)]

use super::types::*;
// use rayon::prelude::*;
// use sdql_runtime::*;

pub fn q16_query(
    customer: &Customer,
    orders: &Orders,
    lineitem: &Lineitem,
    part: &Part,
    supplier: &Supplier,
    partsupp: &Partsupp,
    nation: &Nation,
    region: &Region,
) -> TypeQ16 {
    todo!()
}

pub fn q16_query_rayon(
    customer: &Customer,
    orders: &Orders,
    lineitem: &Lineitem,
    part: &Part,
    supplier: &Supplier,
    partsupp: &Partsupp,
    nation: &Nation,
    region: &Region,
) -> TypeQ16 {
    todo!()
}
