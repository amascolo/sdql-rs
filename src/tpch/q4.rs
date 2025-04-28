#![allow(unused_mut, unused_variables)]

use super::types::*;
// use rayon::prelude::*;
use sdql_runtime::*;

pub fn q4_query(orders: &Orders, lineitem: &Lineitem) -> TypeQ4 {
    let mut l_h: Vec<i32> = (0..lineitem.16)
        .into_iter()
        .filter(|&i| lineitem.11[i as usize] < lineitem.12[i as usize])
        .fold(vec![i32::default(); 6000001], |mut acc: Vec<i32>, i| {
            acc[lineitem.0[i as usize] as usize] += 1i32;
            acc
        });
    let mut o_h: HashMap<VarChar<15>, i32> = (0..orders.9)
        .into_iter()
        .filter(|&i| {
            date!(19930701) <= orders.4[i as usize]
                && orders.4[i as usize] < date!(19931001)
                && l_h[orders.0[i as usize] as usize] > 0i32
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<VarChar<15>, i32>, i| {
                acc[&orders.5[i as usize]] += 1i32;
                acc
            },
        );
    o_h.into_iter()
        .map(|(k, v)| (Record::new((k, v)), TRUE))
        .collect()
}

pub fn q4_query_rayon(_orders: &Orders, _lineitem: &Lineitem) -> TypeQ4 {
    todo!()
}
