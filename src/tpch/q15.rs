#![allow(unused_mut, unused_variables)]

use super::types::*;
// use rayon::prelude::*;
// use sdql_runtime::*;

pub fn q15_query(
    customer: &Customer,
    orders: &Orders,
    lineitem: &Lineitem,
    part: &Part,
    supplier: &Supplier,
    partsupp: &Partsupp,
    nation: &Nation,
    region: &Region,
) -> TypeQ15 {
    todo!()
}

pub fn q15_query_rayon(
    customer: &Customer,
    orders: &Orders,
    lineitem: &Lineitem,
    part: &Part,
    supplier: &Supplier,
    partsupp: &Partsupp,
    nation: &Nation,
    region: &Region,
) -> TypeQ15 {
    todo!()
}
