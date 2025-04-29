#![allow(unused_mut, unused_variables)]

use super::types::*;
// use rayon::prelude::*;
use sdql_runtime::*;

pub fn tpch_09(
    part: &Part,
    supplier: &Supplier,
    lineitem: &Lineitem,
    partsupp: &Partsupp,
    orders: &Orders,
    nation: &Nation,
) -> TypeQ9 {
    let mut n_h: HashMap<i32, Record<(VarChar<25>,)>> = (0..nation.4)
        .into_iter()
        .map(|i| (nation.0[i as usize], Record::new((nation.1[i as usize],))))
        .collect();
    let mut s_h: HashMap<i32, VarChar<25>> = (0..supplier.7)
        .into_iter()
        .map(|i| (supplier.0[i as usize], n_h[&supplier.3[i as usize]].0))
        .collect();
    let mut p_h: HashMap<i32, Record<(i32,)>> = (0..part.9)
        .into_iter()
        .filter(|&i| part.1[i as usize].contains(&"green"))
        .map(|i| (part.0[i as usize], Record::new((part.0[i as usize],))))
        .collect();
    let mut ps_h: HashMap<Record<(i32, i32)>, Record<(VarChar<25>, OrderedFloat<f64>)>> = (0
        ..partsupp.5)
        .into_iter()
        .filter(|&i| p_h.contains_key(&partsupp.0[i as usize]))
        .map(|i| {
            (
                Record::new((partsupp.0[i as usize], partsupp.1[i as usize])),
                Record::new((s_h[&partsupp.1[i as usize]], partsupp.3[i as usize])),
            )
        })
        .collect();
    let mut o_h: Vec<Date> =
        (0..orders.9)
            .into_iter()
            .fold(vec![Date::default(); 6000001], |mut acc: Vec<Date>, i| {
                acc[orders.0[i as usize] as usize] += orders.4[i as usize];
                acc
            });
    let mut l_h: HashMap<Record<(VarChar<25>, i32)>, Record<(OrderedFloat<f64>,)>> = (0..lineitem
        .16)
        .into_iter()
        .filter(|&i| {
            ps_h.contains_key(&Record::new((
                lineitem.1[i as usize],
                lineitem.2[i as usize],
            )))
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<Record<(VarChar<25>, i32)>, Record<(OrderedFloat<f64>,)>>, i| {
                acc[&Record::new((
                    ps_h[&Record::new((lineitem.1[i as usize], lineitem.2[i as usize]))].0,
                    o_h[lineitem.0[i as usize] as usize].year(),
                ))] += Record::new((lineitem.5[i as usize]
                    * (OrderedFloat(1f64) - lineitem.6[i as usize])
                    - ps_h[&Record::new((lineitem.1[i as usize], lineitem.2[i as usize]))].1
                        * lineitem.4[i as usize],));
                acc
            },
        );
    l_h.into_iter()
        .map(|(k, v)| (Record::new((k.0, k.1, v.0)), TRUE))
        .collect()
}

pub fn tpch_09_parallel(
    _part: &Part,
    _supplier: &Supplier,
    _lineitem: &Lineitem,
    _partsupp: &Partsupp,
    _orders: &Orders,
    _nation: &Nation,
) -> TypeQ9 {
    todo!()
}
