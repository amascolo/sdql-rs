#![allow(unused_mut, unused_variables)]

use super::types::*;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_04(orders: &Orders, lineitem: &Lineitem) -> TypeQ4 {
    // TODO update code generator
    // let mut l_h: Vec<Bool> = (0..lineitem.16)
    //     .into_iter()
    //     .filter(|&i| lineitem.11[i as usize] < lineitem.12[i as usize])
    //     .fold(vec![Bool::default(); 6000001], |mut acc: Vec<Bool>, i| {
    //         acc[lineitem.0[i as usize] as usize] += TRUE;
    //         acc
    //     });
    let mut l_h = vec![Bool::default(); 6000001];
    let ptr = l_h.as_mut_ptr() as usize;
    (0..lineitem.16)
        .into_iter()
        .filter(|&i| lineitem.11[i as usize] < lineitem.12[i as usize])
        .for_each(move |i| unsafe {
            *(ptr as *mut Bool).add(lineitem.0[i as usize] as usize) = TRUE;
        });
    let mut o_h: HashMap<VarChar<15>, i32> = (0..orders.9)
        .into_iter()
        .filter(|&i| {
            date!(19930701) <= orders.4[i as usize]
                && orders.4[i as usize] < date!(19931001)
                && l_h[orders.0[i as usize] as usize] == TRUE
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

pub fn tpch_04_parallel(orders: &Orders, lineitem: &Lineitem) -> TypeQ4 {
    // TODO update code generator
    let mut l_h = vec![Bool::default(); 6000001];
    let ptr = l_h.as_mut_ptr() as usize;
    (0..lineitem.16)
        .into_par_iter()
        .filter(|&i| lineitem.11[i as usize] < lineitem.12[i as usize])
        .for_each(move |i| unsafe {
            *(ptr as *mut Bool).add(lineitem.0[i as usize] as usize) = TRUE;
        });
    let mut o_h: HashMap<VarChar<15>, i32> = (0..orders.9)
        .into_par_iter()
        .filter(|&i| {
            date!(19930701) <= orders.4[i as usize]
                && orders.4[i as usize] < date!(19931001)
                && l_h[orders.0[i as usize] as usize] == TRUE
        })
        .fold(HashMap::default, |mut acc: HashMap<VarChar<15>, i32>, i| {
            acc[&orders.5[i as usize]] += 1i32;
            acc
        })
        .sum();
    o_h.into_par_iter()
        .map(|(k, v)| (Record::new((k, v)), TRUE))
        .collect()
}
